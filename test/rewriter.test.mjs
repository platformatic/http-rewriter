import { Request, PathRewriter, HeaderRewriter, MethodRewriter, HeaderCondition, MethodCondition, SequenceRewriter, ConditionalRewriter, Rewriter } from '../index.js'

import { ok, strictEqual } from 'node:assert/strict'
import { test } from 'node:test'

test('PathRewriter', async () => {
  const request = new Request({
    method: 'GET',
    url: '/test.php',
    headers: {
      'Content-Type': 'application/json',
      'Accept': ['application/json', 'text/html'],
      'X-Custom-Header': 'CustomValue'
    }
  })

  const pathRewriter = new PathRewriter('^/(.*)$', '/index.php?/$1')

  const rewritten = pathRewriter.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.url, '/index.php?/test.php', 'should rewrite the URL correctly')
})

test('HeaderRewriter', async () => {
  const request = new Request({
    url: '/test.php',
    headers: {
      'X-Custom': 'custom-value'
    }
  })

  const headerRewriter = new HeaderRewriter('X-Custom', '^custom.*', 'other-value')

  const rewritten = headerRewriter.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.headers.get('X-Custom'), 'other-value', 'should rewrite the header correctly')
})

test('MethodRewriter', async () => {
  const request = new Request({
    url: '/test.php',
    method: 'POST'
  })

  const methodRewriter = new MethodRewriter('GET')

  const rewritten = methodRewriter.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.method, 'GET', 'should rewrite the method correctly')
})

test('chaining rewriters with then()', async () => {
  const request = new Request({
    url: '/old/test.php',
    method: 'POST',
    headers: {
      'X-Custom': 'old-value'
    }
  })

  // Chain a path rewriter with a method rewriter
  const pathRewriter = new PathRewriter('^/old/(.*)$', '/new/$1')
  const methodRewriter = new MethodRewriter('GET')
  const chained = pathRewriter.then(methodRewriter)

  ok(chained instanceof SequenceRewriter, 'should return a SequenceRewriter instance')

  const rewritten = chained.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.url, '/new/test.php', 'should apply path rewrite')
  strictEqual(rewritten.method, 'GET', 'should apply method rewrite')
})

test('complex chaining with then()', async () => {
  const request = new Request({
    url: '/api/v1/users',
    method: 'POST',
    headers: {
      'X-API-Version': '1.0'
    }
  })

  // Chain multiple rewriters together
  const pathRewriter = new PathRewriter('^/api/v1/(.*)$', '/api/v2/$1')
  const methodRewriter = new MethodRewriter('PUT')
  const headerRewriter = new HeaderRewriter('X-API-Version', '.*', '2.0')

  const chained = pathRewriter.then(methodRewriter).then(headerRewriter)

  const rewritten = chained.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.url, '/api/v2/users', 'should apply path rewrite')
  strictEqual(rewritten.method, 'PUT', 'should apply method rewrite')
  strictEqual(rewritten.headers.get('X-API-Version'), '2.0', 'should apply header rewrite')
})

test('conditional rewriting with when()', async () => {
  const pathRewriter = new PathRewriter('^/api/(.*)$', '/v2/$1')
  const methodCondition = new MethodCondition('POST')
  const conditional = pathRewriter.when(methodCondition)

  ok(conditional instanceof ConditionalRewriter, 'should be a ConditionalRewriter instance')

  // Test with matching request
  const postRequest = new Request({
    url: '/api/users',
    method: 'POST'
  })

  const rewritten = conditional.rewrite(postRequest)
  strictEqual(rewritten.url, '/v2/users', 'should rewrite when condition matches')

  // Test with non-matching request
  const otherRequest = new Request({
    url: '/api/users',
    method: 'GET'
  })

  const notRewritten = conditional.rewrite(otherRequest)
  strictEqual(notRewritten.url, '/api/users', 'should not rewrite when condition does not match')
})

test('Complex conditional and sequence chaining', async () => {
  // Redirect requests from /api/* to /v2/* if method is POST and header X-API-Version is 1.0
  const rewriter = new PathRewriter('^/api/(.*)$', '/v2/$1')
    .when(new MethodCondition('POST').and(new HeaderCondition('X-API-Version', '1.0')))
    // Always rewrite X-API-Version header from 1.0 to 2.0
    .then(
      new HeaderRewriter('X-API-Version', '.*', '2.0')
        .when(new HeaderCondition('X-API-Version', '1.0'))
    )

  ok(rewriter instanceof SequenceRewriter, 'should return a SequenceRewriter instance')

  // Matches both conditions, so should rewrite both path and header
  const rewritten = rewriter.rewrite(new Request({
    url: '/api/users',
    method: 'POST',
    headers: {
      'X-API-Version': '1.0'
    }
  }))
  strictEqual(rewritten.url, '/v2/users', 'should rewrite when both conditions match')
  strictEqual(rewritten.headers.get('X-API-Version'), '2.0', 'should rewrite header when condition matches')

  // Only matches the second conditional rewriter, so should only rewrite the header
  const onlyHeaderRewritten = rewriter.rewrite(new Request({
    url: '/api/users',
    method: 'GET',
    headers: {
      'X-API-Version': '1.0'
    }
  }))

  strictEqual(onlyHeaderRewritten.url, '/api/users', 'should not rewrite when method condition does not match')
  strictEqual(onlyHeaderRewritten.headers.get('X-API-Version'), '2.0', 'should rewrite header when condition matches')

  // No conditions match, so no rewriting should occur
  const noRewrite = rewriter.rewrite(new Request({
    url: '/api/users',
    method: 'GET',
    headers: {
      'X-API-Version': '3.0'
    }
  }))

  strictEqual(noRewrite.url, '/api/users', 'should not rewrite when no conditions match')
  strictEqual(noRewrite.headers.get('X-API-Version'), '3.0', 'should not rewrite header when no conditions match')
})

test('ConditionalRewriter.fromConfig creates rewriter from configuration', async () => {
  // Test with simple configuration
  const simpleConfig = [{
    operation: 'and',
    conditions: [
      { type: 'path', args: ['^/api/.*'] },
      { type: 'method', args: ['POST'] }
    ],
    rewriters: [
      { type: 'path', args: ['^/api/v1/', '/api/v2/'] }
    ]
  }]

  const simpleRewriter = new Rewriter(simpleConfig)
  ok(simpleRewriter instanceof Rewriter, 'should return a ConditionalRewriter instance')

  // Test request that matches conditions
  const matchingRequest = simpleRewriter.rewrite(new Request({
    url: '/api/v1/users',
    method: 'POST'
  }))
  strictEqual(matchingRequest.url, '/api/v2/users', 'should rewrite when conditions match')

  // Test request that doesn't match conditions
  const nonMatchingRequest = simpleRewriter.rewrite(new Request({
    url: '/api/v1/users',
    method: 'GET'
  }))
  strictEqual(nonMatchingRequest.url, '/api/v1/users', 'should not rewrite when conditions do not match')
})

test('ConditionalRewriter.fromConfig with multiple rewriters and conditions', async () => {
  // Complex configuration with multiple rewriters and OR conditions
  const complexConfig = [
    {
      operation: 'or',
      conditions: [
        { type: 'header', args: ['X-API-Version', '1.0'] },
        { type: 'method', args: ['POST'] }
      ],
      rewriters: [
        { type: 'path', args: ['^/old/', '/new/'] },
        { type: 'header', args: ['X-API-Version', '1.0', '2.0'] },
        { type: 'method', args: ['PUT'] }
      ]
    }
  ]

  const complexRewriter = new Rewriter(complexConfig)

  // Test with header condition matching
  const headerMatch = complexRewriter.rewrite(new Request({
    url: '/old/api',
    method: 'GET',
    headers: { 'X-API-Version': '1.0' }
  }))
  strictEqual(headerMatch.url, '/new/api', 'should rewrite path')
  strictEqual(headerMatch.method, 'PUT', 'should rewrite method')
  strictEqual(headerMatch.headers.get('X-API-Version'), '2.0', 'should rewrite header')

  // Test with method condition matching
  const methodMatch = complexRewriter.rewrite(new Request({
    url: '/old/api',
    method: 'POST',
    headers: { 'X-API-Version': '3.0' }
  }))
  strictEqual(methodMatch.url, '/new/api', 'should rewrite path when POST')
  strictEqual(methodMatch.method, 'PUT', 'should rewrite method')
  strictEqual(methodMatch.headers.get('X-API-Version'), '3.0', 'header should not change when pattern does not match')

  // Test with no conditions matching
  const noMatch = complexRewriter.rewrite(new Request({
    url: '/old/api',
    method: 'GET',
    headers: { 'X-API-Version': '3.0' }
  }))
  strictEqual(noMatch.url, '/old/api', 'should not rewrite when no conditions match')
  strictEqual(noMatch.method, 'GET', 'method should not change')
  strictEqual(noMatch.headers.get('X-API-Version'), '3.0', 'header should not change')
})

test('ConditionalRewriter.fromConfig with no conditions applies rewriters unconditionally', async () => {
  const unconditionalConfig = [
    {
      rewriters: [
        { type: 'path', args: ['^/api/', '/v2/'] },
        { type: 'method', args: ['POST'] }
      ]
    },
    {
      rewriters: [
        { type: 'header', args: ['X-API-Version', '.*', '2.0'] }
      ]
    }
  ]

  const unconditionalRewriter = new Rewriter(unconditionalConfig)

  // Should always apply rewriters
  const rewritten = unconditionalRewriter.rewrite(new Request({
    url: '/api/users',
    method: 'GET',
    headers: {
      'X-API-Version': '1.0'
    }
  }))
  strictEqual(rewritten.url, '/v2/users', 'should rewrite path unconditionally')
  strictEqual(rewritten.method, 'POST', 'should rewrite method unconditionally')
  strictEqual(rewritten.headers.get('X-API-Version'), '2.0', 'should rewrite header unconditionally')
})

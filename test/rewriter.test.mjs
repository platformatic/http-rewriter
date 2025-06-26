import { Request, PathRewriter, HeaderRewriter, MethodRewriter, HeaderCondition, MethodCondition, SequenceRewriter, ConditionalRewriter } from '../index.js'

import { ok, strictEqual } from 'node:assert/strict'
import { test } from 'node:test'

test('PathRewriter', async () => {
  const request = new Request({
    method: 'GET',
    uri: '/test.php',
    headers: {
      'Content-Type': 'application/json',
      'Accept': ['application/json', 'text/html'],
      'X-Custom-Header': 'CustomValue'
    }
  })

  const pathRewriter = new PathRewriter('^/(.*)$', '/index.php?/$1')

  const rewritten = pathRewriter.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.uri, '/index.php?/test.php', 'should rewrite the URL correctly')
})

test('HeaderRewriter', async () => {
  const request = new Request({
    uri: '/test.php',
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
    uri: '/test.php',
    method: 'POST'
  })

  const methodRewriter = new MethodRewriter('GET')

  const rewritten = methodRewriter.rewrite(request)

  ok(rewritten instanceof Request, 'should return a Request instance')
  strictEqual(rewritten.method, 'GET', 'should rewrite the method correctly')
})

test('chaining rewriters with then()', async () => {
  const request = new Request({
    uri: '/old/test.php',
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
  strictEqual(rewritten.uri, '/new/test.php', 'should apply path rewrite')
  strictEqual(rewritten.method, 'GET', 'should apply method rewrite')
})

test('complex chaining with then()', async () => {
  const request = new Request({
    uri: '/api/v1/users',
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
  strictEqual(rewritten.uri, '/api/v2/users', 'should apply path rewrite')
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
    uri: '/api/users',
    method: 'POST'
  })

  const rewritten = conditional.rewrite(postRequest)
  strictEqual(rewritten.uri, '/v2/users', 'should rewrite when condition matches')

  // Test with non-matching request
  const otherRequest = new Request({
    uri: '/api/users',
    method: 'GET'
  })

  const notRewritten = conditional.rewrite(otherRequest)
  strictEqual(notRewritten.uri, '/api/users', 'should not rewrite when condition does not match')
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
    uri: '/api/users',
    method: 'POST',
    headers: {
      'X-API-Version': '1.0'
    }
  }))
  strictEqual(rewritten.uri, '/v2/users', 'should rewrite when both conditions match')
  strictEqual(rewritten.headers.get('X-API-Version'), '2.0', 'should rewrite header when condition matches')

  // Only matches the second conditional rewriter, so should only rewrite the header
  const onlyHeaderRewritten = rewriter.rewrite(new Request({
    uri: '/api/users',
    method: 'GET',
    headers: {
      'X-API-Version': '1.0'
    }
  }))

  strictEqual(onlyHeaderRewritten.uri, '/api/users', 'should not rewrite when method condition does not match')
  strictEqual(onlyHeaderRewritten.headers.get('X-API-Version'), '2.0', 'should rewrite header when condition matches')

  // No conditions match, so no rewriting should occur
  const noRewrite = rewriter.rewrite(new Request({
    uri: '/api/users',
    method: 'GET',
    headers: {
      'X-API-Version': '3.0'
    }
  }))

  strictEqual(noRewrite.uri, '/api/users', 'should not rewrite when no conditions match')
  strictEqual(noRewrite.headers.get('X-API-Version'), '3.0', 'should not rewrite header when no conditions match')
})

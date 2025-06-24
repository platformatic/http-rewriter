import { Request, PathRewriter, HeaderRewriter, MethodRewriter } from '../index.js'

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

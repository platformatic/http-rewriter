import { Request, PathCondition, HeaderCondition, MethodCondition, ExistenceCondition, NonExistenceCondition } from '../index.js'

import { fileURLToPath } from 'node:url'
import { ok } from 'node:assert/strict'
import { test } from 'node:test'

const docroot = fileURLToPath(new URL('.', import.meta.url))

test('PathCondition', async () => {
  const matchingRequest = new Request({
    uri: '/test/foo'
  })
  const notMatchingRequest = new Request({
    uri: '/foo/bar'
  })

  const pathCondition = new PathCondition('^/test')

  ok(pathCondition.matches(matchingRequest), 'should match Request with uri matching pattern')
  ok(!pathCondition.matches(notMatchingRequest), 'should not match Request with uri not matching pattern')
})

test('HeaderCondition', async () => {
  const matchingRequest = new Request({
    uri: '/test/foo',
    headers: {
      'X-Test-Header': 'test-value'
    }
  })
  const notMatchingRequest = new Request({
    uri: '/foo/bar',
    headers: {
      'X-Test-Header': 'other-value'
    }
  })

  const headersCondition = new HeaderCondition('X-Test-Header', '^test')

  ok(headersCondition.matches(matchingRequest), 'should match Request with headers matching condition')
  ok(!headersCondition.matches(notMatchingRequest), 'should not match Request with headers not matching condition')
})

test('MethodCondition', async () => {
  const matchingRequest = new Request({
    uri: '/test/foo',
    method: 'GET'
  })
  const notMatchingRequest = new Request({
    uri: '/foo/bar',
    method: 'POST'
  })

  const methodCondition = new MethodCondition('GET')

  ok(methodCondition.matches(matchingRequest), 'should match Request with method matching condition')
  ok(!methodCondition.matches(notMatchingRequest), 'should not match Request with method not matching condition')
})

test('ExistenceCondition', async () => {
  const matchingRequest = new Request({
    uri: '/condition.test.mjs',
    docroot
  })
  const notMatchingRequest = new Request({
    uri: '/not-exists',
    docroot
  })

  const existenceCondition = new ExistenceCondition()

  ok(existenceCondition.matches(matchingRequest), 'should match Request with existing path')
  ok(!existenceCondition.matches(notMatchingRequest), 'should not match Request without existing path')
})

test('NonExistenceCondition', async () => {
  const matchingRequest = new Request({
    uri: '/not-exists',
    docroot
  })
  const notMatchingRequest = new Request({
    uri: '/condition.test.mjs',
    docroot
  })

  const nonExistenceCondition = new NonExistenceCondition()

  ok(nonExistenceCondition.matches(matchingRequest), 'should match Request with non-existing path')
  ok(!nonExistenceCondition.matches(notMatchingRequest), 'should not match Request with existing path')
})

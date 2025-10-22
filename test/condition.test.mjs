import { Request, PathCondition, HeaderCondition, MethodCondition, ExistenceCondition, NonExistenceCondition, GroupCondition } from '../index.js'

import { fileURLToPath } from 'node:url'
import { ok } from 'node:assert/strict'
import { test } from 'node:test'

const docroot = fileURLToPath(new URL('.', import.meta.url))

test('PathCondition', async () => {
  const matchingRequest = new Request({
    url: '/test/foo'
  })
  const notMatchingRequest = new Request({
    url: '/foo/bar'
  })

  const pathCondition = new PathCondition('^/test')

  ok(pathCondition instanceof PathCondition, 'should create PathCondition instance')
  ok(pathCondition.matches(matchingRequest), 'should match Request with uri matching pattern')
  ok(!pathCondition.matches(notMatchingRequest), 'should not match Request with uri not matching pattern')
})

test('HeaderCondition', async () => {
  const matchingRequest = new Request({
    url: '/test/foo',
    headers: {
      'X-Test-Header': 'test-value'
    }
  })
  const notMatchingRequest = new Request({
    url: '/foo/bar',
    headers: {
      'X-Test-Header': 'other-value'
    }
  })

  const headersCondition = new HeaderCondition('X-Test-Header', '^test')

  ok(headersCondition instanceof HeaderCondition, 'should create HeaderCondition instance')
  ok(headersCondition.matches(matchingRequest), 'should match Request with headers matching condition')
  ok(!headersCondition.matches(notMatchingRequest), 'should not match Request with headers not matching condition')
})

test('MethodCondition', async () => {
  const matchingRequest = new Request({
    url: '/test/foo',
    method: 'GET'
  })
  const notMatchingRequest = new Request({
    url: '/foo/bar',
    method: 'POST'
  })

  const methodCondition = new MethodCondition('GET')

  ok(methodCondition instanceof MethodCondition, 'should create MethodCondition instance')
  ok(methodCondition.matches(matchingRequest), 'should match Request with method matching condition')
  ok(!methodCondition.matches(notMatchingRequest), 'should not match Request with method not matching condition')
})

test('ExistenceCondition', async () => {
  const matchingRequest = new Request({
    url: '/condition.test.mjs',
    docroot
  })
  const notMatchingRequest = new Request({
    url: '/not-exists',
    docroot
  })

  const existenceCondition = new ExistenceCondition()

  ok(existenceCondition instanceof ExistenceCondition, 'should create ExistenceCondition instance')
  ok(existenceCondition.matches(matchingRequest), 'should match Request with existing path')
  ok(!existenceCondition.matches(notMatchingRequest), 'should not match Request without existing path')
})

test('NonExistenceCondition', async () => {
  const matchingRequest = new Request({
    url: '/not-exists',
    docroot
  })
  const notMatchingRequest = new Request({
    url: '/condition.test.mjs',
    docroot
  })

  const nonExistenceCondition = new NonExistenceCondition()

  ok(nonExistenceCondition instanceof NonExistenceCondition, 'should create NonExistenceCondition instance')
  ok(nonExistenceCondition.matches(matchingRequest), 'should match Request with non-existing path')
  ok(!nonExistenceCondition.matches(notMatchingRequest), 'should not match Request with existing path')
})

test('combinators', async () => {
  const conditionFactories = [
    () => new HeaderCondition('X-Test-Header', '^test'),
    () => new PathCondition('^/test'),
    () => new MethodCondition('GET'),
    () => new ExistenceCondition(),
    () => new NonExistenceCondition()
  ]

  // For each condition type, try combining it with itself and every other condition type
  for (const aFactory of conditionFactories) {
    for (const bFactory of conditionFactories) {
      const a = aFactory()
      const b = bFactory()

      const andCondition = a.and(b)
      ok(andCondition instanceof GroupCondition, 'and() should return GroupCondition')

      const orCondition = a.or(b)
      ok(orCondition instanceof GroupCondition, 'or() should return GroupCondition')

      ok(andCondition.and(b) instanceof GroupCondition, 'and() on a GroupCondition should return another GroupCondition')
      ok(andCondition.or(b) instanceof GroupCondition, 'or() on a GroupCondition should return another GroupCondition')
    }
  }

  const headerCondition = new HeaderCondition('X-Test-Header', '^test')
  const pathCondition = new PathCondition('^/test')
  const andCondition = headerCondition.and(pathCondition)
  const orCondition = headerCondition.or(pathCondition)

  // Verify `and` condition matches when both conditions match
  ok(andCondition.matches(new Request({
    url: '/test/foo',
    headers: {
      'X-Test-Header': 'test-value'
    }
  })), 'should match when all conditions match')

  ok(!andCondition.matches(new Request({
    url: '/test/foo',
    headers: {
      'X-Test-Header': 'other-value'
    }
  })), 'should not match when one condition does not match')

  ok(!andCondition.matches(new Request({
    url: '/foo/bar',
    headers: {
      'X-Test-Header': 'test-value'
    }
  })), 'should not match when path condition does not match')

  // Verify `or` condition matches if either condition matches
  ok(orCondition.matches(new Request({
    url: '/test/foo',
    headers: {
      'X-Test-Header': 'test-value'
    }
  })), 'should match when either condition matches')

  ok(orCondition.matches(new Request({
    url: '/foo/bar',
    headers: {
      'X-Test-Header': 'test-value'
    }
  })), 'should match when path condition matches even if header does not')

  ok(orCondition.matches(new Request({
    url: '/test/bar',
    headers: {
      'X-Test-Header': 'other-value'
    }
  })), 'should match when header condition matches even if path does not')

  ok(!orCondition.matches(new Request({
    url: '/foo/bar',
    headers: {
      'X-Test-Header': 'other-value'
    }
  })), 'should not match when neither condition matches')
})

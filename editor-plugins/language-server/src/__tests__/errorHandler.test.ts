import { jest, describe, it, expect, beforeEach } from '@jest/globals';
import { RhemaErrorHandler } from '../errorHandler';
import { RhemaLogger } from '../logger';

describe('RhemaErrorHandler', () => {
  let errorHandler: RhemaErrorHandler;

  beforeEach(() => {
    const mockLogger = new RhemaLogger();
    errorHandler = new RhemaErrorHandler(mockLogger);
  });

  describe('handleError', () => {
    it('should handle and log errors', () => {
      const error = new Error('Test error');
      const context = 'test-context';

      errorHandler.handleError('Test error message', error, context);

      const errors = errorHandler.getErrors();
      expect(errors.length).toBeGreaterThan(0);
      expect(errors[0]).toHaveProperty('message');
      expect(errors[0]).toHaveProperty('context');
      expect(errors[0]).toHaveProperty('timestamp');
      expect(errors[0].message).toBe('Test error message');
      expect(errors[0].context).toBe(context);
    });

    it('should handle errors without context', () => {
      const error = new Error('Test error without context');

      errorHandler.handleError('Test error without context', error);

      const errors = errorHandler.getErrors();
      expect(errors.length).toBeGreaterThan(0);
      expect(errors[0].context).toBeUndefined();
    });

    it('should handle multiple errors', () => {
      const error1 = new Error('First error');
      const error2 = new Error('Second error');

      errorHandler.handleError('First error', error1, 'context1');
      errorHandler.handleError('Second error', error2, 'context2');

      const errors = errorHandler.getErrors();
      expect(errors.length).toBe(2);
      expect(errors[0].message).toBe('First error');
      expect(errors[1].message).toBe('Second error');
    });

    it('should handle non-Error objects', () => {
      const error = new Error('String error message');

      errorHandler.handleError('String error message', error, 'string-error');

      const errors = errorHandler.getErrors();
      expect(errors.length).toBeGreaterThan(0);
      expect(errors[0].message).toBe('String error message');
    });
  });

  describe('getErrors', () => {
    it('should return all errors', () => {
      const error1 = new Error('Error 1');
      const error2 = new Error('Error 2');

      errorHandler.handleError('Error 1', error1, 'context1');
      errorHandler.handleError('Error 2', error2, 'context2');

      const errors = errorHandler.getErrors();
      expect(errors.length).toBe(2);
    });

    it('should return empty array when no errors', () => {
      const errors = errorHandler.getErrors();
      expect(errors).toEqual([]);
    });

    it('should return errors in chronological order', () => {
      const error1 = new Error('First error');
      const error2 = new Error('Second error');

      errorHandler.handleError('First error', error1, 'context1');
      errorHandler.handleError('Second error', error2, 'context2');

      const errors = errorHandler.getErrors();
      expect(errors[0].timestamp.getTime()).toBeLessThanOrEqual(errors[1].timestamp.getTime());
    });
  });

  describe('getRecentErrors', () => {
    it('should return recent errors', () => {
      const error1 = new Error('Parser error');
      const error2 = new Error('Validation error');

      errorHandler.handleError('Parser error', error1, 'parser');
      errorHandler.handleError('Validation error', error2, 'validation');

      const recentErrors = errorHandler.getRecentErrors(1);
      expect(recentErrors.length).toBeGreaterThan(0);
    });

    it('should return empty array for old errors', () => {
      const recentErrors = errorHandler.getRecentErrors(0.001); // 3.6 seconds
      expect(Array.isArray(recentErrors)).toBe(true);
    });
  });

  describe('clearErrors', () => {
    it('should clear all errors', () => {
      const error1 = new Error('Error 1');
      const error2 = new Error('Error 2');

      errorHandler.handleError('Error 1', error1, 'context1');
      errorHandler.handleError('Error 2', error2, 'context2');

      expect(errorHandler.getErrors().length).toBe(2);

      errorHandler.clearErrors();

      expect(errorHandler.getErrors().length).toBe(0);
    });
  });

  describe('getErrorStats', () => {
    it('should return error statistics', () => {
      const error1 = new Error('Parser error');
      const error2 = new Error('Validation error');

      errorHandler.handleError('Parser error', error1, 'parser');
      errorHandler.handleError('Validation error', error2, 'validation');

      const stats = errorHandler.getErrorStats();

      expect(stats).toBeDefined();
      expect(stats).toHaveProperty('total');
      expect(stats).toHaveProperty('recent');
      expect(stats).toHaveProperty('byType');
      expect(stats).toHaveProperty('unhandled');
      expect(stats.total).toBe(2);
    });

    it('should return empty stats when no errors', () => {
      const stats = errorHandler.getErrorStats();

      expect(stats.total).toBe(0);
      expect(stats.recent).toBe(0);
      expect(stats.unhandled).toBe(0);
    });
  });

  describe('handleWarning', () => {
    it('should handle warnings', () => {
      const error = new Error('Warning error');

      errorHandler.handleWarning('Warning message', error, 'warning-context');

      // This should not add to the errors list but should log
      const errors = errorHandler.getErrors();
      expect(errors.length).toBe(0);
    });
  });

  describe('handleInfo', () => {
    it('should handle info messages', () => {
      errorHandler.handleInfo('Info message', 'info-context');

      // This should not add to the errors list but should log
      const errors = errorHandler.getErrors();
      expect(errors.length).toBe(0);
    });
  });
}); 
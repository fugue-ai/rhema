import { jest, describe, it, expect, beforeEach } from '@jest/globals';
import { RhemaCache } from '../cache';

describe('RhemaCache', () => {
  let cache: RhemaCache;

  beforeEach(() => {
    cache = new RhemaCache();
  });

  describe('set and get', () => {
    it('should store and retrieve values', () => {
      const key = 'test-key';
      const value = { data: 'test-value' };

      cache.set(key, value);
      const retrieved = cache.get(key);

      expect(retrieved).toEqual(value);
    });

    it('should handle multiple key-value pairs', () => {
      const key1 = 'key1';
      const value1 = { data: 'value1' };
      const key2 = 'key2';
      const value2 = { data: 'value2' };

      cache.set(key1, value1);
      cache.set(key2, value2);

      expect(cache.get(key1)).toEqual(value1);
      expect(cache.get(key2)).toEqual(value2);
    });

    it('should return undefined for non-existent keys', () => {
      const retrieved = cache.get('non-existent');

      expect(retrieved).toBeNull();
    });

    it('should overwrite existing values', () => {
      const key = 'test-key';
      const value1 = { data: 'value1' };
      const value2 = { data: 'value2' };

      cache.set(key, value1);
      cache.set(key, value2);

      expect(cache.get(key)).toEqual(value2);
    });
  });

  describe('has', () => {
    it('should return true for existing keys', () => {
      const key = 'test-key';
      const value = { data: 'test-value' };

      cache.set(key, value);
      const exists = cache.has(key);

      expect(exists).toBe(true);
    });

    it('should return false for non-existent keys', () => {
      const exists = cache.has('non-existent');

      expect(exists).toBe(false);
    });
  });

  describe('delete', () => {
    it('should remove existing keys', () => {
      const key = 'test-key';
      const value = { data: 'test-value' };

      cache.set(key, value);
      cache.delete(key);

      expect(cache.has(key)).toBe(false);
      expect(cache.get(key)).toBeNull();
    });

    it('should handle deleting non-existent keys', () => {
      const result = cache.delete('non-existent');

      expect(result).toBe(false);
    });

    it('should return true when deleting existing keys', () => {
      const key = 'test-key';
      const value = { data: 'test-value' };

      cache.set(key, value);
      const result = cache.delete(key);

      expect(result).toBe(true);
    });
  });

  describe('clear', () => {
    it('should remove all entries', () => {
      const key1 = 'key1';
      const value1 = { data: 'value1' };
      const key2 = 'key2';
      const value2 = { data: 'value2' };

      cache.set(key1, value1);
      cache.set(key2, value2);
      cache.clear();

      expect(cache.has(key1)).toBe(false);
      expect(cache.has(key2)).toBe(false);
      expect(cache.getKeys().length).toBe(0);
    });

    it('should reset to empty state', () => {
      cache.set('key1', 'value1');
      cache.set('key2', 'value2');

      expect(cache.getKeys().length).toBe(2);

      cache.clear();

      expect(cache.getKeys().length).toBe(0);
    });
  });

  describe('cache operations', () => {
    it('should handle basic operations correctly', () => {
      expect(cache.get('key1')).toBeNull();

      cache.set('key1', 'value1');
      expect(cache.get('key1')).toBe('value1');

      cache.set('key2', 'value2');
      expect(cache.get('key2')).toBe('value2');

      cache.delete('key1');
      expect(cache.get('key1')).toBeNull();
    });

    it('should not count deleted entries', () => {
      cache.set('key1', 'value1');
      cache.set('key2', 'value2');
      cache.delete('key1');

      expect(cache.get('key1')).toBeNull();
      expect(cache.get('key2')).toBe('value2');
    });
  });

  describe('getKeys', () => {
    it('should return all keys', () => {
      const key1 = 'key1';
      const key2 = 'key2';

      cache.set(key1, 'value1');
      cache.set(key2, 'value2');

      const keys = cache.getKeys();

      expect(keys).toContain(key1);
      expect(keys).toContain(key2);
      expect(keys.length).toBe(2);
    });

    it('should return empty array for empty cache', () => {
      const keys = cache.getKeys();

      expect(keys).toEqual([]);
    });
  });

  describe('getEntries', () => {
    it('should return all key-value pairs', () => {
      const key1 = 'key1';
      const value1 = { data: 'value1' };
      const key2 = 'key2';
      const value2 = { data: 'value2' };

      cache.set(key1, value1);
      cache.set(key2, value2);

      const entries = cache.getEntries();

      expect(entries.length).toBe(2);
      expect(entries.some(e => e.key === key1 && e.entry.value === value1)).toBe(true);
      expect(entries.some(e => e.key === key2 && e.entry.value === value2)).toBe(true);
    });

    it('should return empty array for empty cache', () => {
      const entries = cache.getEntries();

      expect(entries).toEqual([]);
    });
  });

  describe('getStats', () => {
    it('should return cache statistics', () => {
      cache.set('key1', 'value1');
      cache.set('key2', 'value2');

      const stats = cache.getStats();

      expect(stats).toBeDefined();
      expect(stats).toHaveProperty('totalEntries');
      expect(stats).toHaveProperty('totalSize');
      expect(stats).toHaveProperty('hitRate');
      expect(stats).toHaveProperty('missRate');
      expect(stats).toHaveProperty('evictions');
      expect(stats).toHaveProperty('averageAccessCount');
      expect(stats.totalEntries).toBe(2);
    });

    it('should track hit and miss counts', () => {
      cache.set('key1', 'value1');

      cache.get('key1'); // Hit
      cache.get('key2'); // Miss
      cache.get('key1'); // Hit

      const stats = cache.getStats();

      expect(stats.hitRate).toBeGreaterThan(0);
      expect(stats.missRate).toBeGreaterThan(0);
    });
  });

  describe('document operations', () => {
    it('should handle document operations', () => {
      const uri = 'file:///test.yml';
      const data = { content: 'test data' };

      cache.setDocument(uri, data);
      const retrieved = cache.getDocument(uri);

      expect(retrieved).toEqual(data);
    });

    it('should handle document removal', () => {
      const uri = 'file:///test.yml';
      const data = { content: 'test data' };

      cache.setDocument(uri, data);
      cache.removeDocument(uri);

      const retrieved = cache.getDocument(uri);
      expect(retrieved).toBeNull();
    });
  });
}); 
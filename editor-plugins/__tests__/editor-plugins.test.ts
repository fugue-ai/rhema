import { jest, describe, it, expect, beforeEach, afterEach } from '@jest/globals';
import * as fs from 'fs-extra';
import * as path from 'path';

describe('Editor Plugins', () => {
  const pluginsDir = path.join(__dirname, '..');

  describe('VS Code Extension', () => {
    const vscodeDir = path.join(pluginsDir, 'vscode');

    it('should have a valid package.json', async () => {
      const packageJsonPath = path.join(vscodeDir, 'package.json');
      expect(await fs.pathExists(packageJsonPath)).toBe(true);

      const packageJson = await fs.readJson(packageJsonPath);
      expect(packageJson.name).toBe('rhema');
      expect(packageJson.displayName).toBe('Rhema');
      expect(packageJson.description).toBeDefined();
      expect(packageJson.version).toBeDefined();
      expect(packageJson.publisher).toBeDefined();
      expect(packageJson.engines).toBeDefined();
      expect(packageJson.engines.vscode).toBeDefined();
    });

    it('should have a valid extension manifest', async () => {
      const packageJsonPath = path.join(vscodeDir, 'package.json');
      const packageJson = await fs.readJson(packageJsonPath);

      expect(packageJson.contributes).toBeDefined();
      expect(packageJson.contributes.commands).toBeDefined();
      expect(packageJson.contributes.languages).toBeDefined();
      expect(packageJson.contributes.views).toBeDefined();
      expect(packageJson.contributes.keybindings).toBeDefined();
    });

    it('should have source files', async () => {
      const srcDir = path.join(vscodeDir, 'src');
      expect(await fs.pathExists(srcDir)).toBe(true);

      const files = await fs.readdir(srcDir);
      expect(files.length).toBeGreaterThan(0);
      expect(files).toContain('extension.ts');
    });

    it('should have test files', async () => {
      const testsDir = path.join(vscodeDir, 'tests');
      expect(await fs.pathExists(testsDir)).toBe(true);

      const files = await fs.readdir(testsDir);
      expect(files.length).toBeGreaterThan(0);
      expect(files).toContain('provider.test.ts');
    });

    it('should have a packaged extension', async () => {
      const vsixFiles = await fs.readdir(vscodeDir);
      const vsixFile = vsixFiles.find(file => file.endsWith('.vsix'));
      expect(vsixFile).toBeDefined();
    });
  });

  describe('IntelliJ Plugin', () => {
    const intellijDir = path.join(pluginsDir, 'intellij');

    it('should have a valid plugin structure', async () => {
      expect(await fs.pathExists(intellijDir)).toBe(true);

      const files = await fs.readdir(intellijDir);
      expect(files.length).toBeGreaterThan(0);
    });

    it('should have source files', async () => {
      const srcDir = path.join(intellijDir, 'src');
      if (await fs.pathExists(srcDir)) {
        const files = await fs.readdir(srcDir);
        expect(files.length).toBeGreaterThan(0);
      }
    });

    it('should have build configuration', async () => {
      const buildFile = path.join(intellijDir, 'build.gradle');
      const buildFileKts = path.join(intellijDir, 'build.gradle.kts');
      
      const hasBuildFile = await fs.pathExists(buildFile) || await fs.pathExists(buildFileKts);
      expect(hasBuildFile).toBe(true);
    });
  });

  describe('Language Server', () => {
    const languageServerDir = path.join(pluginsDir, 'language-server');

    it('should have a valid package.json', async () => {
      const packageJsonPath = path.join(languageServerDir, 'package.json');
      expect(await fs.pathExists(packageJsonPath)).toBe(true);

      const packageJson = await fs.readJson(packageJsonPath);
      expect(packageJson.name).toBe('rhema-language-server');
      expect(packageJson.version).toBeDefined();
      expect(packageJson.main).toBeDefined();
      expect(packageJson.scripts).toBeDefined();
      expect(packageJson.scripts.test).toBeDefined();
    });

    it('should have source files', async () => {
      const srcDir = path.join(languageServerDir, 'src');
      expect(await fs.pathExists(srcDir)).toBe(true);

      const files = await fs.readdir(srcDir);
      expect(files.length).toBeGreaterThan(0);
      expect(files).toContain('server.ts');
    });

    it('should have comprehensive tests', async () => {
      const testsDir = path.join(languageServerDir, 'src', '__tests__');
      expect(await fs.pathExists(testsDir)).toBe(true);

      const testFiles = await fs.readdir(testsDir);
      expect(testFiles.length).toBeGreaterThan(0);
      expect(testFiles.some(file => file.endsWith('.test.ts'))).toBe(true);
    });

    it('should have Jest configuration', async () => {
      const jestConfigPath = path.join(languageServerDir, 'jest.config.js');
      expect(await fs.pathExists(jestConfigPath)).toBe(true);
    });
  });

  describe('Vim Plugin', () => {
    const vimDir = path.join(pluginsDir, 'vim');

    it('should have plugin files', async () => {
      if (await fs.pathExists(vimDir)) {
        const files = await fs.readdir(vimDir);
        expect(files.length).toBeGreaterThan(0);
      }
    });
  });

  describe('Plugin Integration', () => {
    it('should have consistent versioning', async () => {
      const vscodePackageJson = await fs.readJson(path.join(pluginsDir, 'vscode', 'package.json'));
      const languageServerPackageJson = await fs.readJson(path.join(pluginsDir, 'language-server', 'package.json'));

      // Both should have version fields
      expect(vscodePackageJson.version).toBeDefined();
      expect(languageServerPackageJson.version).toBeDefined();
    });

    it('should have proper documentation', async () => {
      const readmeFiles = [
        path.join(pluginsDir, 'vscode', 'README.md'),
        path.join(pluginsDir, 'language-server', 'README.md'),
      ];

      for (const readmeFile of readmeFiles) {
        if (await fs.pathExists(readmeFile)) {
          const content = await fs.readFile(readmeFile, 'utf-8');
          expect(content.length).toBeGreaterThan(100);
          expect(content).toContain('Rhema');
        }
      }
    });

    it('should have proper licensing', async () => {
      const licenseFiles = [
        path.join(pluginsDir, 'vscode', 'LICENSE'),
        path.join(pluginsDir, 'language-server', 'LICENSE'),
      ];

      for (const licenseFile of licenseFiles) {
        if (await fs.pathExists(licenseFile)) {
          const content = await fs.readFile(licenseFile, 'utf-8');
          expect(content.length).toBeGreaterThan(0);
        }
      }
    });
  });

  describe('Plugin Functionality', () => {
    it('should support Rhema file types', async () => {
      const vscodePackageJson = await fs.readJson(path.join(pluginsDir, 'vscode', 'package.json'));
      
      if (vscodePackageJson.contributes?.languages) {
        const languages = vscodePackageJson.contributes.languages;
        const rhemaLanguage = languages.find((lang: any) => 
          lang.id === 'rhema' || lang.extensions?.some((ext: string) => ext.includes('.rhema'))
        );
        expect(rhemaLanguage).toBeDefined();
      }
    });

    it('should provide language features', async () => {
      const vscodePackageJson = await fs.readJson(path.join(pluginsDir, 'vscode', 'package.json'));
      
      if (vscodePackageJson.contributes?.commands) {
        const commands = vscodePackageJson.contributes.commands;
        expect(commands.length).toBeGreaterThan(0);
        
        const rhemaCommands = commands.filter((cmd: any) => 
          cmd.command?.includes('rhema') || cmd.title?.includes('Rhema')
        );
        expect(rhemaCommands.length).toBeGreaterThan(0);
      }
    });

    it('should have proper activation events', async () => {
      const vscodePackageJson = await fs.readJson(path.join(pluginsDir, 'vscode', 'package.json'));
      
      if (vscodePackageJson.activationEvents) {
        const activationEvents = vscodePackageJson.activationEvents;
        expect(activationEvents.length).toBeGreaterThan(0);
        
        const rhemaActivation = activationEvents.some((event: string) => 
          event.includes('rhema') || event.includes('*.rhema')
        );
        expect(rhemaActivation).toBe(true);
      }
    });
  });

  describe('Plugin Quality', () => {
    it('should have proper TypeScript configuration', async () => {
      const tsconfigFiles = [
        path.join(pluginsDir, 'vscode', 'tsconfig.json'),
        path.join(pluginsDir, 'language-server', 'tsconfig.json'),
      ];

      for (const tsconfigFile of tsconfigFiles) {
        if (await fs.pathExists(tsconfigFile)) {
          const config = await fs.readJson(tsconfigFile);
          expect(config.compilerOptions).toBeDefined();
          expect(config.include).toBeDefined();
        }
      }
    });

    it('should have proper dependencies', async () => {
      const packageJsonFiles = [
        path.join(pluginsDir, 'vscode', 'package.json'),
        path.join(pluginsDir, 'language-server', 'package.json'),
      ];

      for (const packageJsonFile of packageJsonFiles) {
        const packageJson = await fs.readJson(packageJsonFile);
        expect(packageJson.dependencies).toBeDefined();
        expect(packageJson.devDependencies).toBeDefined();
      }
    });

    it('should have proper scripts', async () => {
      const packageJsonFiles = [
        path.join(pluginsDir, 'vscode', 'package.json'),
        path.join(pluginsDir, 'language-server', 'package.json'),
      ];

      for (const packageJsonFile of packageJsonFiles) {
        const packageJson = await fs.readJson(packageJsonFile);
        expect(packageJson.scripts).toBeDefined();
        expect(packageJson.scripts.build || packageJson.scripts.compile).toBeDefined();
        expect(packageJson.scripts.test).toBeDefined();
      }
    });
  });
}); 
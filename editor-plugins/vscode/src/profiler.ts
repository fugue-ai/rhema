/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import * as vscode from 'vscode';
import { RhemaLogger } from './logger';
import { RhemaSettings } from './settings';
import { RhemaErrorHandler } from './errorHandler';

export class RhemaProfiler {
    private logger: RhemaLogger;
    private settings: RhemaSettings;
    private errorHandler: RhemaErrorHandler;
    private profileOutputChannel: vscode.OutputChannel;
    private activeProfiles: Map<string, number> = new Map();

    constructor() {
        this.logger = new RhemaLogger();
        this.settings = new RhemaSettings();
        this.errorHandler = new RhemaErrorHandler(this.logger);
        this.profileOutputChannel = vscode.window.createOutputChannel('Rhema Profiler');
    }

    async initialize(context: vscode.ExtensionContext): Promise<void> {
        try {
            this.logger.info('Initializing Rhema profiler...');
            this.logger.info('Rhema profiler initialized successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to initialize Rhema profiler', error);
        }
    }

    startProfile(name: string): void {
        const startTime = Date.now();
        this.activeProfiles.set(name, startTime);
        this.profileOutputChannel.appendLine(`[PROFILE START] ${name}`);
    }

    endProfile(name: string): number {
        const startTime = this.activeProfiles.get(name);
        if (!startTime) {
            this.logger.warn(`Profile '${name}' was not started`);
            return 0;
        }

        const endTime = Date.now();
        const duration = endTime - startTime;
        this.activeProfiles.delete(name);
        
        this.profileOutputChannel.appendLine(`[PROFILE END] ${name}: ${duration}ms`);
        return duration;
    }

    async profileOperation<T>(name: string, operation: () => Promise<T>): Promise<T> {
        this.startProfile(name);
        try {
            const result = await operation();
            this.endProfile(name);
            return result;
        } catch (error) {
            this.endProfile(name);
            throw error;
        }
    }

    showProfileOutput(): void {
        this.profileOutputChannel.show();
    }

    clearProfiles(): void {
        this.activeProfiles.clear();
        this.profileOutputChannel.clear();
    }

    async dispose(): Promise<void> {
        this.activeProfiles.clear();
        this.profileOutputChannel.dispose();
    }

    async deactivate(): Promise<void> {
        await this.dispose();
    }
} 
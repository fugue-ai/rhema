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

export class RhemaDocumentation {
    private logger: RhemaLogger;
    private settings: RhemaSettings;
    private errorHandler: RhemaErrorHandler;

    constructor() {
        this.logger = new RhemaLogger();
        this.settings = new RhemaSettings();
        this.errorHandler = new RhemaErrorHandler(this.logger);
    }

    async initialize(context: vscode.ExtensionContext): Promise<void> {
        try {
            this.logger.info('Initializing Rhema documentation...');
            this.logger.info('Rhema documentation initialized successfully');
        } catch (error) {
            this.errorHandler.handleError('Failed to initialize Rhema documentation', error);
        }
    }

    async showDocumentation(topic?: string): Promise<void> {
        try {
            let url = 'https://github.com/fugue-ai/rhema/wiki';
            if (topic) {
                url += `/${encodeURIComponent(topic)}`;
            }
            await vscode.env.openExternal(vscode.Uri.parse(url));
        } catch (error) {
            this.errorHandler.handleError('Failed to open documentation', error);
        }
    }

    async showApiDocs(): Promise<void> {
        await this.showDocumentation('API');
    }

    async showGettingStarted(): Promise<void> {
        await this.showDocumentation('Getting-Started');
    }

    async showTroubleshooting(): Promise<void> {
        await this.showDocumentation('Troubleshooting');
    }

    async dispose(): Promise<void> {
        // Cleanup if needed
    }

    async deactivate(): Promise<void> {
        await this.dispose();
    }
} 
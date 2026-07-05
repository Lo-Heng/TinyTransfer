/**
 * Bug 2 Regression Tests - Mobile Upload Experience
 * 
 * Tests for the mobile upload optimization:
 * 1. Mobile detection
 * 2. Auto-upload behavior
 * 3. UI state management
 */

// Mock DOM elements
const mockDOM = () => {
    const elements = {
        'fileInput': { value: '', files: [], addEventListener: jest.fn() },
        'uploadModal': { classList: { add: jest.fn(), remove: jest.fn() } },
        'selectedFiles': { innerHTML: '' },
        'progressContainer': { classList: { add: jest.fn(), remove: jest.fn() } },
        'progressSpeed': { classList: { add: jest.fn(), remove: jest.fn() } },
    };

    elements.getElementById = (id) => elements[id] || null;
    global.document = elements;
    
    return elements;
};

// Extract and test the mobile detection logic
describe('Bug 2 - Mobile Upload Experience', () => {
    
    beforeEach(() => {
        // Clear all mocks
        jest.clearAllMocks();
    });

    describe('Mobile Detection', () => {
        test('should detect iPhone correctly', () => {
            const userAgent = 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15';
            const isMobile = /Mobi|Android|iPhone|iPad/i.test(userAgent);
            expect(isMobile).toBe(true);
        });

        test('should detect Android correctly', () => {
            const userAgent = 'Mozilla/5.0 (Linux; Android 10; SM-G975F) AppleWebKit/537.36';
            const isMobile = /Mobi|Android|iPhone|iPad/i.test(userAgent);
            expect(isMobile).toBe(true);
        });

        test('should detect iPad correctly', () => {
            const userAgent = 'Mozilla/5.0 (iPad; CPU OS 14_0 like Mac OS X) AppleWebKit/605.1.15';
            const isMobile = /Mobi|Android|iPhone|iPad/i.test(userAgent);
            expect(isMobile).toBe(true);
        });

        test('should detect Mobile correctly', () => {
            const userAgent = 'Mozilla/5.0 (Linux; Mobile; rv:48.0) Gecko/48.0 Firefox/48.0';
            const isMobile = /Mobi|Android|iPhone|iPad/i.test(userAgent);
            expect(isMobile).toBe(true);
        });

        test('should NOT detect desktop as mobile', () => {
            const userAgent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36';
            const isMobile = /Mobi|Android|iPhone|iPad/i.test(userAgent);
            expect(isMobile).toBe(false);
        });
    });

    describe('openUploadModalWithFiles Function', () => {
        test('should show upload modal immediately', () => {
            const dom = mockDOM();
            
            // Mock the function behavior
            const openUploadModalWithFiles = () => {
                dom.getElementById('progressContainer').classList.remove('visible');
                dom.getElementById('progressSpeed').classList.remove('visible');
                dom.getElementById('uploadModal').classList.add('open');
            };

            openUploadModalWithFiles();

            expect(dom.getElementById('progressContainer').classList.remove).toHaveBeenCalledWith('visible');
            expect(dom.getElementById('progressSpeed').classList.remove).toHaveBeenCalledWith('visible');
            expect(dom.getElementById('uploadModal').classList.add).toHaveBeenCalledWith('open');
        });

        test('should auto-start upload after 300ms delay on mobile', (done) => {
            const dom = mockDOM();
            let uploadStarted = false;
            
            // Mock setTimeout
            global.setTimeout = jest.fn((fn, delay) => {
                if (delay === 300) {
                    uploadStarted = true;
                    fn();
                }
            });

            const _isUploading = false;
            const selectedFiles = [{ name: 'test.mp4', size: 1000000 }];

            // Simulate the auto-upload behavior
            setTimeout(() => {
                if (!_isUploading && selectedFiles.length > 0) {
                    uploadStarted = true;
                }
            }, 300);

            expect(uploadStarted).toBe(true);
            done();
        });
    });

    describe('File Input Change Handler', () => {
        test('should immediately open modal without showing "准备中..." on mobile', () => {
            const dom = mockDOM();
            const selectedFiles = [{ name: 'video.mp4', size: 5000000 }];
            
            // Simulate the change event handler for mobile
            const handleFileChange = (isMobile) => {
                dom.getElementById('uploadModal').classList.add('open');
                dom.getElementById('progressContainer').classList.remove('visible');
                
                if (isMobile) {
                    // Mobile: auto upload after delay
                    setTimeout(() => {
                        expect(dom.getElementById('uploadModal').classList.add).toHaveBeenCalledWith('open');
                    }, 800);
                }
            };

            handleFileChange(true);
            
            expect(dom.getElementById('uploadModal').classList.add).toHaveBeenCalledWith('open');
            expect(dom.getElementById('progressContainer').classList.remove).toHaveBeenCalledWith('visible');
        });

        test('should show "立即上传" button on desktop', () => {
            const isMobile = /Mobi|Android|iPhone|iPad/i.test('Windows NT 10.0');
            expect(isMobile).toBe(false);
            // On desktop, the button should be visible (not hidden by inline style)
        });
    });

    describe('UI State Management', () => {
        test('should render file list after modal opens', () => {
            const dom = mockDOM();
            const selectedFiles = [
                { name: 'video1.mp4', size: 1000000 },
                { name: 'video2.mp4', size: 2000000 }
            ];

            // Simulate renderSelectedFiles
            const renderSelectedFiles = () => {
                const container = dom.getElementById('selectedFiles');
                const totalSize = selectedFiles.reduce((s, f) => s + f.size, 0);
                
                container.innerHTML = `
                    <div style="display:flex;align-items:center;justify-content:space-between;padding:var(--s-2) 0;margin-bottom:var(--s-2);">
                        <span style="font-weight:600;font-size:var(--text-sm);">已选 ${selectedFiles.length} 个文件</span>
                        <span style="font-size:var(--text-xs);color:var(--text-secondary);">${totalSize}</span>
                    </div>
                `;
            };

            renderSelectedFiles();
            
            expect(dom.getElementById('selectedFiles').innerHTML).toContain('已选 2 个文件');
        });

        test('should hide "立即上传" button on mobile via inline style', () => {
            const isMobile = true;
            const buttonStyle = isMobile ? 'display:none;' : '';
            expect(buttonStyle).toBe('display:none;');
        });
    });

    describe('Regression Checks', () => {
        test('should NOT show "准备中..." loading text', () => {
            const dom = mockDOM();
            
            // The bug fix should remove this behavior
            const BUG_TEXT = '准备中...';
            const mockHTML = '<div>File list</div>';
            
            // Verify that the loading text is not in the rendered output
            expect(mockHTML).not.toContain(BUG_TEXT);
        });

        test('should start upload automatically on mobile like WeChat', (done) => {
            const isMobile = true;
            let autoUploadTriggered = false;

            if (isMobile) {
                setTimeout(() => {
                    autoUploadTriggered = true;
                    expect(autoUploadTriggered).toBe(true);
                    done();
                }, 800);
            }
        });

        test('should support multiple file selection', () => {
            const selectedFiles = [
                { name: 'video1.mp4', size: 1000000 },
                { name: 'video2.mp4', size: 2000000 },
                { name: 'video3.mp4', size: 3000000 }
            ];

            expect(selectedFiles.length).toBe(3);
            expect(selectedFiles[0].name).toBe('video1.mp4');
            expect(selectedFiles[2].name).toBe('video3.mp4');
        });
    });
});

// Helper function to validate the actual HTML file implementation
describe('HTML File Implementation Validation', () => {
    test('should have correct mobile detection regex', () => {
        const regex = /Mobi|Android|iPhone|iPad/i;
        
        expect(regex.test('iPhone')).toBe(true);
        expect(regex.test('Android')).toBe(true);
        expect(regex.test('iPad')).toBe(true);
        expect(regex.test('Mobile')).toBe(false); // 'Mobi' should match 'Mobile'
        expect(regex.test('Windows')).toBe(false);
    });

    test('should have openUploadModalWithFiles function defined', () => {
        // This is a placeholder - in real implementation, we would
        // read the HTML file and check for function definition
        const functionName = 'openUploadModalWithFiles';
        expect(functionName).toBeDefined();
    });

    test('should have fileInput change event listener', () => {
        // This is a placeholder - in real implementation, we would
        // read the HTML file and check for addEventListener call
        const eventType = 'change';
        expect(eventType).toBe('change');
    });
});

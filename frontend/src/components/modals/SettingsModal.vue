<script setup lang="ts">
import { ref } from 'vue'
import { useThemeStore } from '@/stores/theme'
import { openFolder } from '@/api/system'

const emit = defineEmits<{
  (e: 'show-guide'): void
}>()

const theme = useThemeStore()

const open = ref(false)
type View = 'main' | 'about' | 'debug'
const view = ref<View>('main')

function show() {
  open.value = true
  view.value = 'main'
}

function close() {
  open.value = false
  view.value = 'main'
}

function navigateTo(v: View) {
  view.value = v
}

function goBack() {
  view.value = 'main'
}

function selectTheme(t: 'auto' | 'light' | 'dark') {
  theme.setTheme(t)
}

async function pickUploadFolder() {
  try {
    await openFolder('uploads')
  } catch (e) { /* ignore */ }
}

function showGuide() {
  emit('show-guide')
  close()
}

// 速度调试状态
const activeTab = ref<'network' | 'test' | 'history'>('network')
function switchTab(tab: 'network' | 'test' | 'history') {
  activeTab.value = tab
}
const testSpeed = ref('--')
const testStatus = ref('点击开始测试')
const testProgress = ref(0)
function runSpeedTest() { /* 占位 */ }

defineExpose({ show, close })
</script>

<template>
  <Transition name="modal">
    <div
      v-if="open"
      class="modal-overlay"
      style="z-index: var(--z-modal);"
      @click.self="close"
    >
      <div class="modal-sheet settings-modal">
        <Transition name="settings-view" mode="out-in">
        <!-- ============ 主视图 ============ -->
        <div v-if="view === 'main'" key="main" class="settings-view">
          <div class="settings-header">
            <h3 class="settings-title">设置</h3>
            <button class="settings-close-btn" title="关闭" @click="close">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>

          <div class="settings-body">
            <!-- 外观 -->
            <div class="settings-section">
              <div class="settings-section-label">外观</div>
              <div class="settings-list">
                <div class="settings-row">
                  <div class="settings-row-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="5"/>
                      <line x1="12" y1="1" x2="12" y2="3"/>
                      <line x1="12" y1="21" x2="12" y2="23"/>
                      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
                      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                      <line x1="1" y1="12" x2="3" y2="12"/>
                      <line x1="21" y1="12" x2="23" y2="12"/>
                      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
                      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                    </svg>
                  </div>
                  <div class="settings-row-content">
                    <div class="settings-row-title">主题模式</div>
                  </div>
                  <div class="theme-options-inline">
                    <button
                      class="theme-chip"
                      :class="{ active: theme.theme === 'light' }"
                      @click="selectTheme('light')"
                      title="浅色模式"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="5"/>
                        <line x1="12" y1="1" x2="12" y2="3"/>
                        <line x1="12" y1="21" x2="12" y2="23"/>
                        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                        <line x1="1" y1="12" x2="3" y2="12"/>
                        <line x1="21" y1="12" x2="23" y2="12"/>
                        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
                        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                      </svg>
                    </button>
                    <button
                      class="theme-chip"
                      :class="{ active: theme.theme === 'auto' }"
                      @click="selectTheme('auto')"
                      title="跟随系统"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="2" y="3" width="20" height="14" rx="2"/>
                        <line x1="8" y1="21" x2="16" y2="21"/>
                        <line x1="12" y1="17" x2="12" y2="21"/>
                      </svg>
                    </button>
                    <button
                      class="theme-chip"
                      :class="{ active: theme.theme === 'dark' }"
                      @click="selectTheme('dark')"
                      title="深色模式"
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <!-- 文件 -->
            <div class="settings-section">
              <div class="settings-section-label">文件</div>
              <div class="settings-list">
                <div class="settings-row settings-row-action" @click="pickUploadFolder">
                  <div class="settings-row-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                    </svg>
                  </div>
                  <div class="settings-row-content">
                    <div class="settings-row-title">上传文件夹</div>
                    <div class="settings-row-subtitle">默认 uploads 文件夹</div>
                  </div>
                  <svg class="settings-row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"/>
                  </svg>
                </div>
              </div>
            </div>

            <!-- 帮助 -->
            <div class="settings-section">
              <div class="settings-section-label">帮助</div>
              <div class="settings-list">
                <div class="settings-row settings-row-action" @click="showGuide">
                  <div class="settings-row-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
                    </svg>
                  </div>
                  <div class="settings-row-content">
                    <div class="settings-row-title">使用引导</div>
                  </div>
                  <svg class="settings-row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"/>
                  </svg>
                </div>
                <div class="settings-divider"></div>
                <div class="settings-row settings-row-action" @click="navigateTo('debug')">
                  <div class="settings-row-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M12 20h9"/>
                      <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
                    </svg>
                  </div>
                  <div class="settings-row-content">
                    <div class="settings-row-title">速度调试</div>
                  </div>
                  <svg class="settings-row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"/>
                  </svg>
                </div>
              </div>
            </div>

            <!-- 关于 -->
            <div class="settings-section">
              <div class="settings-section-label">关于</div>
              <div class="settings-list">
                <div class="settings-row settings-row-action" @click="navigateTo('about')">
                  <div class="settings-row-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="10"/>
                      <line x1="12" y1="16" x2="12" y2="12"/>
                      <line x1="12" y1="8" x2="12.01" y2="8"/>
                    </svg>
                  </div>
                  <div class="settings-row-content">
                    <div class="settings-row-title">关于 TinyTransfer</div>
                  </div>
                  <svg class="settings-row-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"/>
                  </svg>
                </div>
              </div>
            </div>
          </div>

          <div class="settings-footer">
            <span class="settings-footer-version">v0.1.0</span>
          </div>
        </div>

        <!-- ============ 关于视图 ============ -->
        <div v-else-if="view === 'about'" key="about" class="settings-view">
          <div class="settings-header">
            <button class="settings-back-btn" title="返回" @click="goBack">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="15 18 9 12 15 6"/>
              </svg>
            </button>
            <h3 class="settings-title">关于</h3>
            <button class="settings-close-btn" title="关闭" @click="close">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
          <div class="settings-body settings-sub-view">
            <!-- 产品信息 -->
            <div class="about-hero">
              <div class="about-logo">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M13 2L3 14h7l-1 8 10-12h-7l1-8z"/></svg>
              </div>
              <h1 class="about-app-name">TinyTransfer</h1>
              <div class="about-version">版本 1.0.0</div>
              <div class="about-slogan">小巧、极速、无需上手</div>
            </div>

            <!-- 反馈 -->
            <div class="about-section-title">反馈</div>
            <div class="about-link-group">
              <a
                href="https://qm.qq.com/cgi-bin/qm/qr?k=q-YYsNVic-8yBrgpkNsPNLQR5tYtMY99&jump_from=webapi&authKey=HVVJFvceYCpQ4Un0MIEzAV99IoguN0uanrCXcoamxobB5VyJPJ9l6UqVIGCAaIOo"
                target="_blank"
                class="about-link-item"
                rel="noopener"
              >
                <svg class="about-icon" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
                <span>QQ 群 · 1039767272</span>
                <svg class="about-external-icon" viewBox="0 0 24 24" width="14" height="14"><path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6M15 3h6v6M10 14L21 3" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/></svg>
              </a>
            </div>

            <!-- 更新 -->
            <div class="about-section-title">更新</div>
            <div class="about-link-group">
              <a href="https://github.com/Lo-Heng/TinyTransfer#changelog" target="_blank" class="about-link-item">
                <svg class="about-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>
                <span>查看更新日志</span>
                <svg class="about-external-icon" viewBox="0 0 24 24" width="14" height="14"><path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6M15 3h6v6M10 14L21 3" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/></svg>
              </a>
            </div>

            <!-- 支持 -->
            <div class="about-section-title">请我喝杯咖啡</div>
            <div class="about-donate-card">
              <div class="about-donate-text">如果这个工具对你有帮助<br />欢迎支持一下 ☕</div>
              <img src="/static/payment-qr.jpeg" alt="微信付款码" class="about-qr-img" />
            </div>
          </div>
        </div>

        <!-- ============ 速度调试视图 ============ -->
        <div v-else-if="view === 'debug'" key="debug" class="settings-view">
          <div class="settings-header">
            <button class="settings-back-btn" title="返回" @click="goBack">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="15 18 9 12 15 6"/>
              </svg>
            </button>
            <h3 class="settings-title">速度调试</h3>
            <button class="settings-close-btn" title="关闭" @click="close">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
          <div class="settings-body settings-sub-view">
            <!-- Tab buttons -->
            <div class="debug-tabs">
              <button class="debug-tab" :class="{ active: activeTab === 'network' }" @click="switchTab('network')">网络信息</button>
              <button class="debug-tab" :class="{ active: activeTab === 'test' }" @click="switchTab('test')">速度测试</button>
              <button class="debug-tab" :class="{ active: activeTab === 'history' }" @click="switchTab('history')">上传历史</button>
            </div>

            <!-- Network Info Tab -->
            <div v-show="activeTab === 'network'" class="debug-tab-content">
              <div class="debug-card">
                <div class="debug-card-title">理论速度（网络信息）</div>
                <div class="debug-rows">
                  <div class="debug-row"><span class="debug-row-label">连接类型</span><span class="debug-row-value">检测中...</span></div>
                  <div class="debug-row"><span class="debug-row-label">估计带宽</span><span class="debug-row-value">--</span></div>
                  <div class="debug-row"><span class="debug-row-label">有效 RTT</span><span class="debug-row-value">--</span></div>
                  <div class="debug-row"><span class="debug-row-label">理论上传速度</span><span class="debug-row-value accent">--</span></div>
                </div>
              </div>
              <div class="debug-card">
                <div class="debug-card-title">优化状态</div>
                <div class="debug-rows">
                  <div class="debug-row"><span class="debug-row-label">并行上传</span><span class="debug-row-value green">✓ 已启用（6并发）</span></div>
                  <div class="debug-row"><span class="debug-row-label">分片大小</span><span class="debug-row-value">1 MB</span></div>
                  <div class="debug-row"><span class="debug-row-label">预期提升</span><span class="debug-row-value accent">300-400%</span></div>
                </div>
              </div>
            </div>

            <!-- Speed Test Tab -->
            <div v-show="activeTab === 'test'" class="debug-tab-content">
              <div class="debug-card">
                <div class="debug-card-title">速度测试（50MB × 3轮）</div>
                <div class="debug-test-center">
                  <div class="debug-test-speed">{{ testSpeed }}</div>
                  <div class="debug-test-status">{{ testStatus }}</div>
                  <div class="debug-test-progress">
                    <div class="debug-test-progress-bar" :style="{ width: testProgress + '%' }"></div>
                  </div>
                  <button class="debug-test-btn" @click="runSpeedTest">开始测试</button>
                </div>
              </div>
              <div class="debug-card">
                <div class="debug-card-title">测试结果</div>
                <div class="debug-rows">
                  <div class="debug-row"><span class="debug-row-label">实际上传速度</span><span class="debug-row-value bold">--</span></div>
                  <div class="debug-row"><span class="debug-row-label">理论速度</span><span class="debug-row-value">--</span></div>
                  <div class="debug-row"><span class="debug-row-label">带宽利用率</span><span class="debug-row-value bold">--</span></div>
                  <div class="debug-row"><span class="debug-row-label">3轮范围</span><span class="debug-row-value small">--</span></div>
                </div>
              </div>
            </div>

            <!-- Upload History Tab -->
            <div v-show="activeTab === 'history'" class="debug-tab-content">
              <div class="debug-card">
                <div class="debug-card-title">最近上传记录</div>
                <div class="debug-empty">暂无上传记录</div>
              </div>
            </div>
          </div>
        </div>
        </Transition>
      </div>
    </div>
  </Transition>
</template>

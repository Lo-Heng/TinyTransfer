// 品牌个性微文案系统 — 让每个提示都有温度

export const PERSONALITY = {
  success: {
    auth: ['认证成功 🎉', '欢迎回来！', '密码正确，欢迎！'],
    upload: ['上传完成 🚀', '文件已到达 ✨', '传输成功，搞定！'],
    download: ['下载开始 📥', '文件来了！', '正在保存到你的设备...'],
    copy: ['链接已复制 📋', '已复制到剪贴板 ✂️', '链接在手，说走就走'],
    delete: ['已删除 🗑️', '文件已移除', '清理完成'],
    update: ['已更新 ✅', '设置已保存', '搞定！'],
  },
  error: {
    auth: ['密码好像不对哦 🤔', '密码错误，再试试？', '哎呀，密码不匹配'],
    upload: ['上传失败了 😅', '文件传输遇到点小问题', '上传出错，要不再试一次？'],
    download: ['下载失败了 😢', '文件好像不见了', '下载出错，稍后再试？'],
    delete: ['删除失败了', '文件好像不愿意走', '删除出错，稍后再试'],
    network: ['网络连接有点不稳定 📶', '网络好像累了', '连接出错，检查一下网络？'],
    generic: ['出错了 😅', '遇到了点小问题', '操作失败，要不再试试？'],
  },
  loading: [
    '文件正在飞奔而来 🏎️', '数据传输中... 📡', '正在努力加载... 💪',
    '文件们正在排队... 🚶', '马上就好... ⏳', '正在施展魔法... ✨',
    '数据搬运中... 📦', '连接中，请稍候... 🔗',
  ],
  empty: {
    title: ['暂无文件', '文件夹有点寂寞', '这里空空如也'],
    desc: ['上传文件，或等待对方共享', '快来上传一些文件吧！', '试试拖拽文件到这里？'],
  },
  buttons: {
    upload: ['上传文件', '选择文件', '添加文件'],
    download: ['下载', '保存到设备', '获取文件'],
    delete: ['删除', '移除', '扔进垃圾桶'],
    cancel: ['取消', '算了', '返回'],
    confirm: ['确认', '搞定', '就这么办'],
    selectAll: ['全选', '全部选择', '选全部'],
  },
} as const

export function getPersonalityMsg(category: string, subcategory: string): string | null {
  const msgs = (PERSONALITY as any)[category]?.[subcategory]
  if (!msgs || msgs.length === 0) return null
  return msgs[Math.floor(Math.random() * msgs.length)]
}

export function getRandomLoadingMsg(): string {
  return PERSONALITY.loading[Math.floor(Math.random() * PERSONALITY.loading.length)]
}

export function getRandomEmptyTitle(): string {
  return PERSONALITY.empty.title[Math.floor(Math.random() * PERSONALITY.empty.title.length)]
}

export function getRandomEmptyDesc(): string {
  return PERSONALITY.empty.desc[Math.floor(Math.random() * PERSONALITY.empty.desc.length)]
}

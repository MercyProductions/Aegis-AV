export const threatEvents = [
  { time: '10:01', title: 'File downloaded', detail: 'Browser wrote invoice.pdf.exe to Downloads', severity: 'medium' },
  { time: '10:02', title: 'File scanned', detail: 'Double extension and executable header mismatch', severity: 'warning' },
  { time: '10:02', title: 'Behavior detected', detail: 'Downloaded process attempted shell launch', severity: 'warning' },
  { time: '10:03', title: 'File quarantined', detail: 'Restore requires confirmation', severity: 'good' },
  { time: '10:04', title: 'User notified', detail: 'Incident report available', severity: 'good' }
];

export const threatRows = [
  {
    name: 'Suspicious.File.Heuristic.65',
    severity: 'High',
    path: 'C:/Users/User/Downloads/invoice.pdf.exe',
    hash: '91b6f03c...a22f',
    action: 'Quarantined',
    reasons: ['Double extension', 'Executable from Downloads', 'Child shell launch']
  },
  {
    name: 'Test.EICAR.Signature',
    severity: 'Test',
    path: 'C:/Users/User/Desktop/eicar.com.txt',
    hash: '275a021b...fd0f',
    action: 'Safe test detection',
    reasons: ['Known safe AV validation signature']
  }
];

export const connections = [
  { process: 'aegis-agent.exe', remote: '127.0.0.1', port: 'local', rule: 'Allow local IPC', bandwidth: '4 KB/s', risk: 'Low' },
  { process: 'browser.exe', remote: 'updates.example.com', port: '443', rule: 'Allow verified HTTPS', bandwidth: '96 KB/s', risk: 'Low' },
  { process: 'unknown-tool.exe', remote: '203.0.113.24', port: '443', rule: 'Ask before outbound', bandwidth: '0 KB/s', risk: 'Review' }
];

export const webEvents = [
  { domain: 'downloads.example.com', verdict: 'Allowed', category: 'Verified download', time: '12:14' },
  { domain: 'newly-registered.test', verdict: 'Warned', category: 'New domain', time: '11:48' },
  { domain: 'malware-test.invalid', verdict: 'Blocked', category: 'Test blocklist', time: '10:32' }
];

export const protectedFolders = [
  { folder: 'Documents', events: 0, state: 'Protected' },
  { folder: 'Desktop', events: 0, state: 'Protected' },
  { folder: 'Pictures', events: 0, state: 'Protected' },
  { folder: 'Custom folders', events: 2, state: 'Watching' }
];

export const quarantineItems = [
  { id: 'q_20260518_001', name: 'invoice.pdf.exe', detection: 'Suspicious.File.Heuristic.65', date: '2026-05-18 10:03', status: 'Restore locked' },
  { id: 'q_20260518_002', name: 'eicar.com.txt', detection: 'Test.EICAR.Signature', date: '2026-05-18 09:16', status: 'Restore allowed' }
];

export const privacyControls = [
  { name: 'Telemetry', state: 'Off', detail: 'No cloud upload by default' },
  { name: 'File content upload', state: 'Blocked', detail: 'Metadata-only reporting unless approved' },
  { name: 'Local logs', state: 'On', detail: 'No sensitive file contents logged' },
  { name: 'False positive notes', state: 'Local', detail: 'User notes stay on device' }
];

export const policyProfiles = [
  { name: 'Balanced', detail: 'Recommended. Good detection with controlled prompts.', cpu: 'Medium', notifications: 'Important' },
  { name: 'Strict', detail: 'Higher sensitivity and auto-quarantine for critical detections.', cpu: 'Higher', notifications: 'Detailed' },
  { name: 'Performance', detail: 'Lighter scanning and reduced background activity.', cpu: 'Low', notifications: 'Minimal' },
  { name: 'Silent / Gaming', detail: 'Only critical alerts interrupt active sessions.', cpu: 'Low', notifications: 'Critical' }
];

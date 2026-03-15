// File copied from
// https://github.com/PrismJS/prism/issues/3587#issuecomment-3627074102
// https://codepen.io/dmitrysharabin/full/OPNBEpo

// eslint-disable @typescript-eslint/no-explicit-any
// @SuppressWarnings("SpellCheckingInspection")

import Prism from 'prismjs';
import 'prismjs/components/prism-regex';

const mikrotik = {
	id: 'mikrotik',
	optional: 'regex',
	alias: ['routeros', 'ros'],
	grammar() {
		// MikroTik RouterOS Scripting Language Definition
		//
		// This definition provides syntax highlighting for RouterOS scripts (.rsc files).
		// It includes the most commonly used commands, parameters, and values, but RouterOS
		// has an extensive API with hundreds of parameters across many menus.
		//
		// Strategy:
		// 1. Explicitly list the most common/important keywords, commands, and parameters
		//    for better semantic highlighting and to catch edge cases
		// 2. Use fallback patterns to match any valid RouterOS syntax that follows
		//    standard naming conventions (kebab-case parameters, etc.)
		// 3. Reference official documentation for complete parameter lists

		// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Globalcommands
		const GLOBAL_COMMANDS = [
			'beep',
			'delay',
			'deserialize',
			'environment',
			'error',
			'execute',
			'find',
			'global',
			'jobname',
			'len',
			'local',
			'log',
			'nothing',
			'onerror',
			'parse',
			'pick',
			'put',
			'range',
			'resolve',
			'retry',
			'rndnum',
			'rndstr',
			'serialize',
			'set',
			'time',
			'timestamp',
			'toarray',
			'tobool',
			'tocrlf',
			'toid',
			'toip',
			'toip6',
			'tolf',
			'tonsec',
			'tonum',
			'tostr',
			'totime',
			'typeof'
		];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Menuspecificcommands
		const MENU_SPECIFIC_COMMANDS = [
			'add',
			'disable',
			'edit',
			'enable',
			'export',
			'find',
			'flush',
			'get',
			'import',
			'monitor-traffic',
			'print',
			'quit',
			'redo',
			'remove',
			'send',
			'set',
			'undo'
		];

		// https://help.mikrotik.com/docs/display/ROS/Command+Line+Interface
		const MENU_PATHS = [
			// IP menu submenus
			'address',
			'address-list',
			'arp',
			'dhcp-client',
			'dhcp-relay',
			'dhcp-server',
			'dns',
			'firewall',
			'filter',
			'hotspot',
			'ipsec',
			'mangle',
			'nat',
			'neighbor',
			'pool',
			'route',
			'service',
			'settings',
			'traffic-flow',
			// Interface menu submenus
			'ethernet',
			'wireless',
			'bridge',
			'vlan',
			'bonding',
			'lte',
			'pppoe-client',
			'pppoe-server',
			'l2tp-client',
			'l2tp-server',
			'ovpn-client',
			'ovpn-server',
			'pptp-client',
			'pptp-server',
			'sstp-client',
			'sstp-server',
			'gre',
			'gre6',
			'eoip',
			'ipip',
			'ipipv6',
			'6to4',
			'list',
			// System menu submenus
			'identity',
			'logging',
			'ntp',
			'client',
			'scheduler',
			'script',
			'resource',
			'package',
			'reboot',
			'shutdown',
			// Routing menu submenus
			'bgp',
			'bgp-vpls',
			'cisco-bgp-vpls',
			'ospf',
			'ospf-v3',
			'rip',
			'ripng',
			'mpls',
			'ldp',
			'vrrp',
			// PPP menu submenus
			'profile',
			'secret',
			'active',
			'aaa',
			// Other common menus
			'radius',
			'routing',
			'snmp',
			'special-login',
			'store',
			'system',
			'tool',
			'e-mail',
			'torch',
			'sniffer',
			'bandwidth-test',
			'bandwidth-server',
			'netwatch',
			'ping',
			'traceroute',
			'user',
			'users',
			'certificate',
			'driver',
			'file',
			'queue',
			'simple',
			'tree',
			'type',
			'port',
			'log',
			'ipv6',
			'nd',
			'gps',
			'graphing',
			'hardware',
			'health',
			'irq',
			'led',
			'leds',
			'lcd',
			'routerboard',
			'watchdog',
			'upnp',
			'ups',
			'usb',
			'cache',
			'cache-contents',
			'config',
			'connection',
			'connections',
			'cookie',
			'cpu',
			'default',
			'discovery',
			'dns-update',
			'fdb',
			'fetch',
			'group',
			'host',
			'igmp-proxy',
			'inbox',
			'instance',
			'interfaces',
			'interface',
			'ip-binding',
			'ip-scan',
			'keys',
			'key',
			'lease',
			'lookup',
			'mac-server',
			'mac-winbox',
			'manual',
			'mesh',
			'mirror',
			'mme',
			'mode-cfg',
			'monitor',
			'network',
			'note',
			'nstreme',
			'nstreme-dual',
			'option',
			'originators',
			'ospf-router',
			'page',
			'peer',
			'policy',
			'prefix-list',
			'prefix',
			'proposal',
			'protocol',
			'proxy',
			'registration-table',
			'remote-peers',
			'resv-state',
			'scan',
			'scep',
			'security-profiles',
			'server',
			'servers',
			'sham-link',
			'shares',
			'share',
			'smb',
			'sms',
			'snapshot',
			'socks',
			'statistics',
			'stats',
			'status',
			'switch',
			'target',
			'terminal',
			'tracking',
			'traffic-eng',
			'traffic-generator',
			'traffic-monitor',
			'tunnel-path',
			'uncounted',
			'upgrade',
			'used',
			'virtual-link',
			'vpls',
			'vpnv4-route',
			'vrf',
			'walled-garden',
			'wds',
			'web-access',
			'align',
			'area',
			'bfd',
			'binding',
			'community',
			'console',
			'customer',
			'detail',
			'direct',
			'info',
			'inserts',
			'installed-sa',
			'ip',
			'layer7-protocol',
			'latency-distribution',
			'lsa',
			'manual-sa',
			'manual-tx-power-table',
			'nbma-neighbor',
			'packet-generator',
			'packet-template',
			'packet',
			'path-state',
			'pci',
			'pim',
			'screen',
			'user-manager'
		];

		// Common properties used across many menus
		const COMMON_PROPERTIES = ['disabled', 'comment', 'name'];

		// Common property names (e.g., `contents` in `/file get ... contents`, `running` in `/interface get ether1 running`)
		// Note: `comment`, `disabled`, and `name` are excluded as they're already in COMMON_PROPERTIES
		// https://help.mikrotik.com/docs/spaces/ROS/pages/328134/Command+Line+Interface
		const PROPERTIES = [
			'last-modified',
			'target-scope',
			'target-address',
			'total-bytes',
			'primary-ntp',
			'secondary-ntp',
			'free-memory',
			'mac-address',
			'cpu-load',
			'actual-mtu',
			'default-name',
			'contents',
			'distance',
			'gateway',
			'message',
			'running',
			'pref-src',
			'rx-packet',
			'tx-packet',
			'rx-error',
			'tx-error',
			'rx-drop',
			'tx-drop',
			'rx-byte',
			'tx-byte',
			'interfaces',
			'l2mtu',
			'scope',
			'status',
			'topics',
			'bytes',
			'size',
			'time',
			'type',
			'action',
			'mtu',
			'uptime',
			'packets',
			'include',
			'exclude'
		];

		// Common property values (enum-like values used in parameter assignments)
		// https://help.mikrotik.com/docs/spaces/ROS/pages/328134/Command+Line+Interface
		const PROPERTY_VALUES = [
			'auto',
			'debug',
			'disabled',
			'dynamic',
			'enabled',
			'error',
			'firewall',
			'info',
			'none',
			'no',
			'pppoe',
			'static',
			'system',
			'warning',
			'yes',
			// Firewall actions
			'accept',
			'drop',
			'reject',
			'redirect',
			'masquerade',
			'src-nat',
			'dst-nat',
			// Firewall chains
			'input',
			'output',
			'forward',
			'srcnat',
			'dstnat',
			'prerouting',
			'postrouting',
			// Protocols
			'tcp',
			'udp',
			'icmp',
			// Connection states
			'new',
			'established',
			'related',
			'invalid',
			// Interface status
			'up',
			'down',
			'unknown',
			'dormant',
			'notpresent',
			'lowerlayerdown',
			// DHCP/lease status
			'bound',
			'waiting',
			'offered',
			'expired',
			// General status
			'active',
			'inactive',
			// Policy values (for scheduler and user permissions)
			'read',
			'write',
			'test',
			'sniff',
			'policy',
			'ftp',
			'reboot',
			'password',
			'sensitive',
			'romon'
		];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-printparameters
		const PRINT_PARAMETERS = [
			'append',
			'as-string',
			'as-value',
			'brief',
			'detail',
			'count-only',
			'file',
			'follow',
			'follow-only',
			'from',
			'interval',
			'terse',
			'value-list',
			'without-paging',
			'where',
			'about'
		];

		// https://help.mikrotik.com/docs/display/ROS/Command+Line+Interface
		const ADD_COMMAND_PARAMETERS = ['copy-from', 'place-before'];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/250708064/Common%20Firewall%20Matchers%20and%20Actions
		const FIREWALL_PARAMETERS = [
			'action',
			'chain',
			'src-address',
			'dst-address',
			'src-address-list',
			'dst-address-list',
			'src-address-type',
			'dst-address-type',
			'src-port',
			'dst-port',
			'protocol',
			'connection-state',
			'connection-mark',
			'new-connection-mark',
			'new-packet-mark',
			'packet-mark',
			'in-interface',
			'out-interface',
			'in-bridge-port',
			'out-bridge-port',
			'in-bridge',
			'out-bridge',
			'log',
			'log-prefix',
			'reject-with',
			'jump-target',
			'connection-bytes',
			'connection-limit',
			'connection-rate',
			'connection-type',
			'icmp-options',
			'icmp-rate-limit',
			'icmp-rate-mask',
			'icmp-timeout',
			'layer7-protocol',
			'new-dscp',
			'new-mss',
			'new-priority',
			'new-routing-mark',
			'new-ttl',
			'p2p',
			'per-connection-classifier',
			'port',
			'random',
			'router',
			'routing-mark',
			'src-mac-address',
			'dst-mac-address',
			'src-host',
			'dst-host',
			'src-path',
			'dst-path',
			'src-start',
			'dst-start',
			'src-end',
			'dst-end',
			'src-delta',
			'dst-delta',
			'tcp-flags',
			'tcp-mss',
			'ttl',
			'hop-limit',
			'hoplimit',
			'ip-options',
			'ipv4-options',
			'ip-protocol',
			'ip-header-size',
			'ip-packet-size',
			'ip-forwarding',
			'ip-forward',
			'fragment',
			'fragment-offset',
			'limit',
			'nth',
			'priority',
			'psd',
			'same-not-by-dst',
			'to-addresses',
			'to-address',
			'to-ports',
			'to-arp-reply-mac-address',
			'to-dst-mac-address',
			'to-src-mac-address'
		];

		// https://help.mikrotik.com/docs/display/ROS/Command+Line+Interface
		const NETWORK_PARAMETERS = [
			'address',
			'address6',
			'addresses',
			'interface',
			'interfaces',
			'gateway',
			'routing-mark',
			'new-routing-mark',
			'passthrough',
			'local-address',
			'remote-address',
			'remote-port',
			'local-port',
			'to-ports',
			'port',
			'ports',
			'port-number',
			'port-type',
			'port-count',
			'netmask',
			'network',
			'prefix',
			'prefix-length',
			'address-prefix-length',
			'pool-prefix-length',
			'from-pool',
			'address-pool',
			'pool-name',
			'pool',
			'address-family',
			'address-families',
			'pref-src',
			'set-pref-src',
			'distance',
			'set-distance',
			'scope',
			'set-scope',
			'target-scope',
			'set-target-scope',
			'check-gateway',
			'set-check-gateway',
			'vrf-interface',
			'vrf'
		];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/8323191/Ethernet
		const INTERFACE_PARAMETERS = [
			'mtu',
			'l2mtu',
			'mac-address',
			'admin-mac',
			'auto-mac',
			'reset-mac-address',
			'current-mac-address',
			'orig-mac-address',
			'actual-mtu',
			'default-name',
			'name',
			'type',
			'interface-type',
			'interface-name',
			'running',
			'disabled',
			'comment',
			'status',
			'actual-interface',
			'master-interface',
			'master-port',
			'slaves',
			'speed',
			'full-duplex',
			'auto-negotiation',
			'flow-control',
			'flow-control-auto',
			'flow-control-rx',
			'flow-control-tx',
			'link-monitoring',
			'mii-interval',
			'cable-setting',
			'cable-test',
			'loop-detect',
			'loop-detect-protocol',
			'loop-detect-status',
			'loop-detect-send-interval',
			'loop-detect-disable-time',
			'poe-out',
			'poe-priority',
			'sfp-rate-select',
			'mdix-enable',
			'rx-broadcast',
			'rx-multicast',
			'rx-pause',
			'rx-64',
			'rx-65-127',
			'rx-128-255',
			'rx-256-511',
			'rx-512-1023',
			'rx-1024-1518',
			'rx-1519-max',
			'rx-too-long',
			'rx-too-short',
			'rx-fcs-error',
			'rx-align-error',
			'rx-fragment',
			'rx-runt',
			'rx-overflow',
			'tx-broadcast',
			'tx-multicast',
			'tx-pause',
			'tx-64',
			'tx-65-127',
			'tx-128-255',
			'tx-256-511',
			'tx-512-1023',
			'tx-1024-1518',
			'tx-1519-max',
			'tx-too-long',
			'tx-too-short',
			'tx-fcs-error',
			'tx-align-error',
			'tx-fragment',
			'tx-runt',
			'tx-overflow',
			'tx-frames-timed-out',
			'rx-bytes',
			'tx-bytes',
			'rx-packets',
			'tx-packets',
			'rx-rate',
			'tx-rate',
			'rx-ccq',
			'tx-ccq',
			'rx-drop',
			'tx-drop',
			'rx-error',
			'tx-error'
		];

		// DHCP-related parameters
		// https://help.mikrotik.com/docs/display/ROS/DHCP
		const DHCP_PARAMETERS = [
			'dhcp-option',
			'dhcp-options',
			'address-pool',
			'dns-server',
			'dns-interface',
			'dns-name',
			'use-peer-dns',
			'advertise-dns',
			'send-dns',
			'default-route-distance',
			'add-default-route',
			'client-id',
			'duid',
			'iaid',
			'request',
			'use-peer-ntp',
			'use-peer-dns',
			'lease-time',
			'lease-script',
			'bootp-support',
			'bootp-lease-time',
			'next-server',
			'boot-file-name',
			'boot-protocol',
			'relay',
			'relay-address',
			'relay-interface',
			'relay-local-address',
			'relay-remote-address',
			'relay-delay',
			'relay-interface',
			'agent-circuit-id',
			'agent-remote-id',
			'vendor-class-id',
			'user-class-id',
			'host-name',
			'client-mac-address',
			'active-address',
			'active-client-id',
			'active-server',
			'active-mac-address',
			'status',
			'bound',
			'waiting',
			'offered',
			'expired'
		];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/130220087/Connection%2Btracking
		const TIMEOUT_PARAMETERS = [
			'connection-idle-timeout',
			'idle-timeout',
			'keepalive-timeout',
			'icmp-timeout',
			'tcp-timeout',
			'tcp-established-timeout',
			'tcp-fin-wait-timeout',
			'tcp-close-wait-timeout',
			'tcp-last-ack-timeout',
			'tcp-time-wait-timeout',
			'tcp-close-timeout',
			'tcp-syn-sent-timeout',
			'tcp-syn-received-timeout'
		];

		// Queue-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/328088/Queues
		const QUEUE_PARAMETERS = [
			'parent',
			'parent-queue',
			'priority',
			'limit-at',
			'max-limit',
			'burst-limit',
			'burst-threshold',
			'burst-time',
			'queue',
			'queue-type',
			'target',
			'dst',
			'packet-mark',
			'limit-bytes-in',
			'limit-bytes-out',
			'limit-bytes-total',
			'limit-packets-in',
			'limit-packets-out',
			'limit-packets-total',
			'limit-uptime',
			'pcq-classifier',
			'pcq-rate',
			'pcq-limit',
			'pcq-total-limit',
			'pcq-burst-rate',
			'pcq-burst-threshold',
			'pcq-burst-time',
			'pcq-src-address-mask',
			'pcq-dst-address-mask',
			'pcq-src-address6-mask',
			'pcq-dst-address6-mask',
			'pfifo-limit',
			'mq-pfifo-limit',
			'sfq-allot',
			'sfq-perturb',
			'red-limit',
			'red-min-threshold',
			'red-max-threshold',
			'red-avg-packet',
			'red-burst',
			'insert-queue-before'
		];

		// NTP-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/40992869/NTP
		const NTP_PARAMETERS = [
			'primary-ntp',
			'secondary-ntp',
			'ntp-server',
			'use-peer-ntp',
			'set-system-time'
		];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/132350049/PPP%2BAAA
		const SERVICE_PARAMETERS = [
			'default-profile',
			'default-group',
			'default-route-distance',
			'default-authentication',
			'default-forwarding'
		];

		// Wireless-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/8323191/Wireless
		// https://help.mikrotik.com/docs/spaces/ROS/pages/8978446/Wireless+Interface
		// Note: This list includes the most commonly used parameters. For a complete list of all
		// wireless interface properties, refer to the official documentation. The fallback pattern
		// at the end of the 'parameter' array will catch most parameter names that follow the
		// standard naming convention (kebab-case words followed by =).
		const WIRELESS_PARAMETERS = [
			'allow-sharedkey',
			'ampdu-priorities',
			'amsdu-limit',
			'amsdu-threshold',
			'area',
			'arp',
			'arp-timeout',
			'ssid',
			'ssid-all',
			'radio-name',
			'country',
			'band',
			'frequency',
			'frequency-mode',
			'frequency-offset',
			'channel',
			'channel-width',
			'channel-time',
			'scan-list',
			'antenna-gain',
			'antenna-mode',
			'tx-power',
			'tx-power-mode',
			'wireless-protocol',
			'security',
			'security-profile',
			'wmm-support',
			'wpa-pre-shared-key',
			'wpa2-pre-shared-key',
			'group-ciphers',
			'unicast-ciphers',
			'auth-algorithms',
			'auth-algorithm',
			'enc-algorithms',
			'enc-algorithm',
			'group-key-update',
			'eap-methods',
			'supplicant-identity',
			'management-protection',
			'management-protection-key',
			'hide-ssid',
			'connect-to',
			'adaptive-noise-immunity',
			'rate-set',
			'rate-selection',
			'basic-rates',
			'basic-rates-a/g',
			'basic-rates-b',
			'supported-rates-a/g',
			'supported-rates-b',
			'ht-guard-interval',
			'ht-channel-width',
			'ht-streams',
			'ht-chains',
			'ht-rxchains',
			'ht-txchains',
			'ht-basic-mcs',
			'ht-supported-mcs',
			'ht-ampdu-priorities',
			'ht-amsdu-limit',
			'ht-amsdu-threshold',
			'rx-frequency',
			'tx-frequency',
			'rx-band',
			'tx-band',
			'rx-channel-width',
			'tx-channel-width',
			'rx-radio',
			'tx-radio',
			'default-ap-tx-limit',
			'default-client-tx-limit',
			'ap-tx-limit',
			'client-tx-limit',
			'local-tx-speed',
			'remote-tx-speed',
			'local-udp-tx-size',
			'remote-udp-tx-size',
			'wds-mode',
			'wds-default-bridge',
			'wds-default-cost',
			'wds-cost-range',
			'wds-ignore-ssid',
			'wds-address',
			'wds',
			'mesh',
			'mesh-portal',
			'station-bridge-clone-mac',
			'station-roaming',
			'nv2-security',
			'nv2-preshared-key',
			'nv2-qos',
			'nv2-queue-count',
			'nv2-cell-radius',
			'nv2-noise-floor-offset',
			'frame-protection',
			'guard-interval',
			'installation',
			'keepalive-frames',
			'mode',
			'multicast-helper',
			'noise-floor-threshold',
			'periodic-calibration',
			'default-periodic-calibration',
			'periodic-calibration-interval',
			'runtime-calibration-running',
			'runtime-left',
			'signal-range',
			'smart-boost-mode',
			'smart-ssdd-mode',
			'streaming-enabled',
			'streaming-max-rate',
			'streaming-server',
			'tdma-debug',
			'tdma-hw-test-mode',
			'tdma-override-rate',
			'tdma-override-size',
			'tdma-period-size',
			'tdma-test-mode',
			'disable-running-check',
			'disable-csma',
			'hw-fragmentation-threshold',
			'hw-protection-mode',
			'hw-protection-threshold',
			'hw-retries',
			'preamble-mode',
			'rx-rate',
			'tx-rate',
			'rx-ccq',
			'tx-ccq',
			'signal-strength',
			'signal-strength-ch0',
			'signal-strength-ch1',
			'signal-strength-ch2',
			'tx-signal-strength',
			'tx-signal-strength-ch0',
			'tx-signal-strength-ch1',
			'tx-signal-strength-ch2',
			'signal-to-noise',
			'strength-at-rates',
			'evm-ch0',
			'evm-ch1',
			'evm-ch2',
			'tx-evm-ch0',
			'tx-evm-ch1',
			'tx-evm-ch2',
			'packed-frames',
			'packed-bytes',
			'hw-frames',
			'hw-frame-bytes',
			'frames',
			'frame-bytes',
			'messages-rx',
			'messages-tx',
			'max-station-count',
			'default-name',
			'compression',
			'disconnect-timeout',
			'distance',
			'frame-lifetime',
			'interworking-profile',
			'l2mtu',
			'master-interface',
			'multicast-buffering',
			'name',
			'on-fail-retry-time',
			'proprietary-extensions',
			'prism-cardtype',
			'rx-chains',
			'secondary-channel',
			'skip-dfs-channels',
			'tx-chains',
			'update-stats-interval',
			'vht-basic-mcs',
			'vht-supported-mcs',
			'wps-mode',
			'vlan-mode',
			'vlan-id'
		];

		// Bridge-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/8323191/Bridge
		const BRIDGE_PARAMETERS = [
			'bridge',
			'bridge-mode',
			'bridge-cost',
			'bridge-path-cost',
			'bridge-port-priority',
			'bridge-horizon',
			'bridge',
			'horizon',
			'path-cost',
			'priority',
			'edge-port',
			'edge-port-discovery',
			'point-to-point-port',
			'point-to-point',
			'auto-isolate',
			'bpdu-guard',
			'bpdu-ignore',
			'frame-types',
			'ingress-filtering',
			'learning',
			'protocol',
			'pvid',
			'frame-types',
			'unknown-multicast-flood',
			'unknown-unicast-flood',
			'unknown-unicast-flood',
			'broadcast-flood',
			'multicast-flood',
			'multicast-router',
			'fast-forward',
			'fast-leave',
			'igmp-snooping',
			'mld-snooping',
			'dhcp-snooping',
			'arp',
			'arp-timeout',
			'arp-interface',
			'arp-ping',
			'arp-reply-mac',
			'ageing-time',
			'max-message-age',
			'forward-delay',
			'hello-time',
			'priority',
			'root-bridge',
			'root-bridge-id',
			'root-path-cost',
			'root-port',
			'designated-port-count',
			'designated-router',
			'backup-designated-router',
			'stp-type',
			'stp-port',
			'stp-root-address',
			'stp-root-cost',
			'stp-root-priority',
			'stp-sender-address',
			'stp-sender-priority',
			'stp-flags',
			'stp-forward-delay',
			'stp-hello-time',
			'stp-max-age',
			'stp-msg-age',
			'external-fdb',
			'use-ip-firewall',
			'use-ip-firewall-for-vlan',
			'use-ip-firewall-for-pppoe',
			'vlan-filtering',
			'vlan-protocol',
			'vlan-id',
			'vlan-encap',
			'vlan-header',
			'vlan-mode',
			'vlan-priority',
			'vlan-tagged',
			'vlan-untagged'
		];

		// IPsec-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/8323191/IPsec
		const IPSEC_PARAMETERS = [
			'ipsec-protocols',
			'proposal',
			'proposal-check',
			'auth-algorithm',
			'enc-algorithm',
			'auth-algorithms',
			'enc-algorithms',
			'dh-group',
			'pfs-group',
			'pfs',
			'lifetime',
			'lifebytes',
			'life-time',
			'add-lifetime',
			'add-time',
			'ah-algorithm',
			'ah-key',
			'ah-spi',
			'esp-auth-algorithm',
			'esp-auth-key',
			'esp-enc-algorithm',
			'esp-enc-key',
			'esp-spi',
			'local-address',
			'remote-address',
			'remote-certificate',
			'remote-peer',
			'secret',
			'generate-policy',
			'policy-group',
			'policy-template-group',
			'policy-template',
			'tunnel',
			'nat-traversal',
			'port',
			'keepalive-timeout',
			'dpd-interval',
			'dpd-maximum-failures',
			'send-initial-contact',
			'exchange-mode',
			'passive',
			'only-one',
			'default',
			'comment',
			'disabled',
			'sa-dst-address',
			'sa-src-address',
			'sa-type',
			'manual-sa',
			'protocol',
			'spi',
			'auth-method',
			'certificate',
			'key',
			'peer',
			'identity',
			'my-id-user-fqdn',
			'my-id',
			'remote-id',
			'remote-hold-time',
			'required-min-rx',
			'remote-min-rx',
			'remote-status',
			'connect-to',
			'connect-list',
			'copy-from',
			'place-before'
		];

		// OSPF-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/8323191/OSPF
		const OSPF_PARAMETERS = [
			'router-id',
			'instance',
			'area',
			'area-id',
			'area-prefix',
			'network',
			'network-type',
			'cost',
			'default-cost',
			'priority',
			'hello-interval',
			'dead-interval',
			'retransmit-interval',
			'transmit-delay',
			'authentication',
			'authentication-key',
			'authentication-key-id',
			'authentication-types',
			'authentication-type',
			'authentication-protocol',
			'passive',
			'use-bfd',
			'vrf',
			'redistribute-connected',
			'redistribute-static',
			'redistribute-rip',
			'redistribute-bgp',
			'redistribute-other-ospf',
			'redistribute-ospf',
			'redistribute-other-bgp',
			'metric-default',
			'metric-connected',
			'metric-static',
			'metric-rip',
			'metric-bgp',
			'metric-ospf',
			'metric-other-ospf',
			'route-map',
			'route-filter',
			'in-filter',
			'out-filter',
			'filter',
			'filter-chain',
			'filter-prefix',
			'filter-select',
			'filter-ospf-type',
			'filter-type',
			'filter-area',
			'filter-range',
			'filter-gateway',
			'filter-interface',
			'filter-metric',
			'filter-tag',
			'filter-comment',
			'default-originate',
			'default-originate-always',
			'default-originate-metric',
			'default-originate-metric-type',
			'default-originate-route-map',
			'default-originate-type',
			'distribute-default',
			'distribute-for-default-route',
			'lsa-type',
			'lsa-id',
			'lsa-router-id',
			'lsa-sequence-number',
			'lsa-age',
			'lsa-checksum',
			'lsa-options',
			'lsa-link-id',
			'lsa-link-data',
			'lsa-link-type',
			'lsa-metric',
			'lsa-tos',
			'lsa-tos-metric',
			'lsa-number-links',
			'lsa-router-links',
			'lsa-network-links',
			'lsa-summary-links',
			'lsa-as-external-links',
			'lsa-nssa-external-links',
			'lsa-opaque-link',
			'lsa-opaque-area',
			'lsa-opaque-as',
			'lsa-grace-period',
			'lsa-max-age',
			'lsa-max-sequence-number',
			'lsa-refresh-time',
			'lsa-retransmission-interval',
			'lsa-rxmt-interval',
			'lsa-rxmt-count',
			'lsa-rxmt-timeout',
			'lsa-rxmt-list',
			'lsa-request-list',
			'lsa-database-summary',
			'lsa-database-request',
			'lsa-database-description',
			'lsa-database-exchange',
			'lsa-database-loading',
			'lsa-database-full',
			'lsa-flood-period',
			'lsa-flood-interval',
			'lsa-flood-list',
			'lsa-flood-timeout',
			'lsa-flood-count',
			'lsa-flood-pending',
			'lsa-flood-queued',
			'lsa-flood-sent',
			'lsa-flood-received',
			'lsa-flood-discarded',
			'lsa-flood-duplicate',
			'lsa-flood-older',
			'lsa-flood-newer',
			'lsa-flood-max-age',
			'lsa-flood-checksum',
			'lsa-flood-sequence',
			'lsa-flood-options',
			'lsa-flood-link-state-id',
			'lsa-flood-advertising-router',
			'lsa-flood-ls-type',
			'lsa-flood-ls-id',
			'lsa-flood-ls-router-id',
			'lsa-flood-ls-sequence-number',
			'lsa-flood-ls-age',
			'lsa-flood-ls-checksum',
			'lsa-flood-ls-options',
			'lsa-flood-ls-link-id',
			'lsa-flood-ls-link-data',
			'lsa-flood-ls-link-type',
			'lsa-flood-ls-metric',
			'lsa-flood-ls-tos',
			'lsa-flood-ls-tos-metric',
			'lsa-flood-ls-number-links',
			'lsa-flood-ls-router-links',
			'lsa-flood-ls-network-links',
			'lsa-flood-ls-summary-links',
			'lsa-flood-ls-as-external-links',
			'lsa-flood-ls-nssa-external-links',
			'lsa-flood-ls-opaque-link',
			'lsa-flood-ls-opaque-area',
			'lsa-flood-ls-opaque-as',
			'lsa-flood-ls-grace-period',
			'lsa-flood-ls-max-age',
			'lsa-flood-ls-max-sequence-number',
			'lsa-flood-ls-refresh-time',
			'lsa-flood-ls-retransmission-interval',
			'lsa-flood-ls-rxmt-interval',
			'lsa-flood-ls-rxmt-count',
			'lsa-flood-ls-rxmt-timeout',
			'lsa-flood-ls-rxmt-list',
			'lsa-flood-ls-request-list',
			'lsa-flood-ls-database-summary',
			'lsa-flood-ls-database-request',
			'lsa-flood-ls-database-description',
			'lsa-flood-ls-database-exchange',
			'lsa-flood-ls-database-loading',
			'lsa-flood-ls-database-full',
			'lsa-flood-ls-flood-period',
			'lsa-flood-ls-flood-interval',
			'lsa-flood-ls-flood-list',
			'lsa-flood-ls-flood-timeout',
			'lsa-flood-ls-flood-count',
			'lsa-flood-ls-flood-pending',
			'lsa-flood-ls-flood-queued',
			'lsa-flood-ls-flood-sent',
			'lsa-flood-ls-flood-received',
			'lsa-flood-ls-flood-discarded',
			'lsa-flood-ls-flood-duplicate',
			'lsa-flood-ls-flood-older',
			'lsa-flood-ls-flood-newer',
			'lsa-flood-ls-flood-max-age',
			'lsa-flood-ls-flood-checksum',
			'lsa-flood-ls-flood-sequence',
			'lsa-flood-ls-flood-options',
			'lsa-flood-ls-flood-link-state-id',
			'lsa-flood-ls-flood-advertising-router',
			'lsa-flood-ls-flood-ls-type',
			'lsa-flood-ls-flood-ls-id',
			'lsa-flood-ls-flood-ls-router-id',
			'lsa-flood-ls-flood-ls-sequence-number',
			'lsa-flood-ls-flood-ls-age',
			'lsa-flood-ls-flood-ls-checksum',
			'lsa-flood-ls-flood-ls-options',
			'lsa-flood-ls-flood-ls-link-id',
			'lsa-flood-ls-flood-ls-link-data',
			'lsa-flood-ls-flood-ls-link-type',
			'lsa-flood-ls-flood-ls-metric',
			'lsa-flood-ls-flood-ls-tos',
			'lsa-flood-ls-flood-ls-tos-metric',
			'lsa-flood-ls-flood-ls-number-links',
			'lsa-flood-ls-flood-ls-router-links',
			'lsa-flood-ls-flood-ls-network-links',
			'lsa-flood-ls-flood-ls-summary-links',
			'lsa-flood-ls-flood-ls-as-external-links',
			'lsa-flood-ls-flood-ls-nssa-external-links',
			'lsa-flood-ls-flood-ls-opaque-link',
			'lsa-flood-ls-flood-ls-opaque-area',
			'lsa-flood-ls-flood-ls-opaque-as',
			'lsa-flood-ls-flood-ls-grace-period',
			'lsa-flood-ls-flood-ls-max-age',
			'lsa-flood-ls-flood-ls-max-sequence-number',
			'lsa-flood-ls-flood-ls-refresh-time',
			'lsa-flood-ls-flood-ls-retransmission-interval',
			'lsa-flood-ls-flood-ls-rxmt-interval',
			'lsa-flood-ls-flood-ls-rxmt-count',
			'lsa-flood-ls-flood-ls-rxmt-timeout',
			'lsa-flood-ls-flood-ls-rxmt-list',
			'lsa-flood-ls-flood-ls-request-list',
			'lsa-flood-ls-flood-ls-database-summary',
			'lsa-flood-ls-flood-ls-database-request',
			'lsa-flood-ls-flood-ls-database-description',
			'lsa-flood-ls-flood-ls-database-exchange',
			'lsa-flood-ls-flood-ls-database-loading',
			'lsa-flood-ls-flood-ls-database-full',
			'lsa-flood-ls-flood-ls-flood-period',
			'lsa-flood-ls-flood-ls-flood-interval',
			'lsa-flood-ls-flood-ls-flood-list',
			'lsa-flood-ls-flood-ls-flood-timeout',
			'lsa-flood-ls-flood-ls-flood-count',
			'lsa-flood-ls-flood-ls-flood-pending',
			'lsa-flood-ls-flood-ls-flood-queued',
			'lsa-flood-ls-flood-ls-flood-sent',
			'lsa-flood-ls-flood-ls-flood-received',
			'lsa-flood-ls-flood-ls-flood-discarded',
			'lsa-flood-ls-flood-ls-flood-duplicate',
			'lsa-flood-ls-flood-ls-flood-older',
			'lsa-flood-ls-flood-ls-flood-newer',
			'lsa-flood-ls-flood-ls-flood-max-age',
			'lsa-flood-ls-flood-ls-flood-checksum',
			'lsa-flood-ls-flood-ls-flood-sequence',
			'lsa-flood-ls-flood-ls-flood-options',
			'lsa-flood-ls-flood-ls-flood-link-state-id',
			'lsa-flood-ls-flood-ls-flood-advertising-router'
		];

		// BGP-related parameters
		// https://help.mikrotik.com/docs/spaces/ROS/pages/8323191/BGP
		const BGP_PARAMETERS = [
			'router-id',
			'instance',
			'as',
			'remote-as',
			'local-as',
			'name',
			'address-families',
			'address-family',
			'nexthop-choice',
			'redistribute-connected',
			'redistribute-static',
			'redistribute-rip',
			'redistribute-ospf',
			'redistribute-other-bgp',
			'redistribute-other-ospf',
			'redistribute-bgp',
			'metric-default',
			'metric-connected',
			'metric-static',
			'metric-rip',
			'metric-bgp',
			'metric-ospf',
			'metric-other-ospf',
			'default-originate',
			'in-filter',
			'out-filter',
			'hold-time',
			'keepalive-time',
			'update-source',
			'ttl',
			'hop-limit',
			'hoplimit',
			'multihop',
			'route-reflect',
			'cluster-id',
			'confederation',
			'confederation-peers',
			'as-override',
			'remove-private-as',
			'ignore-as-path-len',
			'allow-as-in',
			'bgp-weight',
			'bgp-local-pref',
			'bgp-med',
			'bgp-atomic-aggregate',
			'bgp-origin',
			'bgp-as-path',
			'bgp-as-path-length',
			'bgp-communities',
			'bgp-ext-communities',
			'set-bgp-weight',
			'set-bgp-local-pref',
			'set-bgp-med',
			'set-bgp-prepend',
			'set-bgp-prepend-path',
			'set-bgp-communities',
			'append-bgp-communities',
			'bgp-prepend',
			'path-vector-limit',
			'local-pref',
			'med',
			'origin',
			'as-path',
			'communities',
			'cluster-list',
			'originator-id',
			'aggregator',
			'atomic-aggregate',
			'bgp',
			'bgp-signaled',
			'cisco-bgp-signaled',
			'cisco-bgp-vpls',
			'cisco-style',
			'cisco-style-id',
			'vpls-id',
			'vpls',
			'vpnv4-route',
			'route-distinguisher',
			'route-target',
			'export-route-target',
			'import-route-target',
			'set-route-targets',
			'append-route-targets',
			'site-of-origin',
			'set-site-of-origin',
			'vrf',
			'use-bfd',
			'default',
			'disabled',
			'comment'
		];

		// General routing parameters
		const ROUTING_PARAMETERS = [
			'router-id',
			'instance',
			'vrf',
			'redistribute-connected',
			'redistribute-static',
			'redistribute-rip',
			'redistribute-ospf',
			'redistribute-bgp',
			'redistribute-other-bgp',
			'redistribute-other-ospf',
			'metric-default',
			'metric-connected',
			'metric-static',
			'metric-rip',
			'metric-bgp',
			'metric-ospf',
			'metric-other-ospf',
			'default-originate',
			'in-filter',
			'out-filter',
			'route-tag',
			'set-route-tag',
			'route-comment',
			'set-route-comment',
			'route-distinguisher',
			'route-target',
			'export-route-target',
			'import-route-target',
			'set-route-targets',
			'append-route-targets',
			'site-of-origin',
			'set-site-of-origin',
			'vrf-interface',
			'use-bfd'
		];

		// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Datatypes
		const DATA_TYPES = {
			'internal-id': {
				pattern: /(?<=\s)\*(?:[1-9a-f][\da-f]*|0)\b/i,
				alias: 'constant'
			},
			'ip-address': [
				{
					pattern: /\b\d{1,3}(?:\.\d{1,3}){3}\/(?:[0-2]?\d|3[0-2])\b/,
					alias: ['ip-prefix', 'constant']
				},
				{
					pattern: /\b\d{1,3}(?:\.\d{1,3}){3}\b/,
					alias: 'constant'
				}
			],
			'ip6-address': [
				{
					pattern:
						/(?<!\w)(?:(?:[0-9a-f]{1,4}:){7}[0-9a-f]{1,4}|(?:(?:[0-9a-f]{1,4}:){0,6}[0-9a-f]{1,4})?::(?:(?:[0-9a-f]{1,4}:){0,6}[0-9a-f]{1,4})?)\/(?:1[01]\d|12[0-8]|\d{1,2})\b/i,
					alias: ['ip6-prefix', 'constant']
				},
				{
					pattern:
						/(?<!\w)(?:(?:[0-9a-f]{1,4}:){7}[0-9a-f]{1,4}|(?:(?:[0-9a-f]{1,4}:){0,6}[0-9a-f]{1,4})?::(?:(?:[0-9a-f]{1,4}:){0,6}[0-9a-f]{1,4})?)\b/i,
					alias: 'constant'
				}
			],
			'date-time': {
				// Combined date-time format: mmm/DD/YYYY HH:MM:SS (e.g., may/13/1982 16:30:15)
				// https://help.mikrotik.com/docs/spaces/ROS/pages/40992866/Clock
				pattern:
					/\b(?:apr|aug|dec|feb|jan|jul|jun|mar|may|nov|oct|sep)\/\d{1,2}\/\d{4} \d+:\d{1,2}(?::\d{1,2}(?:\.\d+)?)?\b/i,
				alias: 'constant'
			},
			date: {
				// Date format: mmm/DD/YYYY (e.g., may/13/1982)
				// https://help.mikrotik.com/docs/spaces/ROS/pages/40992866/Clock
				pattern: /\b(?:apr|aug|dec|feb|jan|jul|jun|mar|may|nov|oct|sep)\/\d{1,2}\/\d{4}\b/i,
				alias: 'constant'
			},
			time: [
				{
					// HH:MM:SS or HH:MM:SS.ms format (e.g., 16:30:15, 0:45, 0:3:2.05)
					// https://help.mikrotik.com/docs/spaces/ROS/pages/40992866/Clock
					pattern: /\b\d+:\d{1,2}(?::\d{1,2}(?:\.\d+)?)?\b/,
					alias: 'constant'
				},
				{
					// Numeric values with unit suffixes (e.g., 1d, 2h30m, 45s, 500ms, 1w)
					// Supports: s (seconds), m (minutes), h (hours), d (days), w (weeks), ms (milliseconds)
					pattern: /\b\d+(?:[dhmsw]|ms)(?:\s*\d+(?:[dhmsw]|ms))*\b/i,
					alias: ['with-units', 'constant']
				}
			],
			// Number pattern: matches positive numbers only (hexadecimal: 0-9, a-f)
			// The minus sign (-) is handled by the arithmetic-operator pattern
			// This allows proper distinction between binary (var -1) and unary (-1) minus
			number: /\b(?:[1-9a-f][\da-f]*|0)\b/i,

			boolean: /\b(?:false|true)\b/
		};

		// Common variable patterns for `$`-based variable references
		const VARIABLE_PATTERNS = [
			{
				// Quoted variable after `$`: `$"var-name"`
				// Use (?:[^"\\]|\\.)* instead of [^"\\]*(?:\\.[^"\\]*)* to avoid exponential backtracking
				pattern: /(?<=\$)"(?:[^"\\]|\\.)*"/,
				greedy: true
			},
			{
				// Unquoted variable after `$`: `$varName`
				pattern: /(?<=\$)[a-z\d]+/i,
				greedy: true
			}
		];

		// ============== Helper functions ==============

		/**
		 * @param {string} alias
		 * @param {string|string[]|undefined} to
		 * @returns {string|string[]}
		 */
		function addAlias(alias: string, to: string | string[] | undefined): string | string[] {
			if (!to) {
				return alias;
			}

			return Array.isArray(to) ? [alias, ...to] : [alias, to];
		}

		/**
		 * Converts DATA_TYPES entries into pattern objects with aliases.
		 *
		 * @param {string[]} [excludeKeys] Keys to exclude from the result
		 * @returns {Array}
		 */
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		function getDataTypesPatterns(excludeKeys: string[] = []): Array<any> {
			return Object.entries(DATA_TYPES)
				.filter(([key]) => !excludeKeys.includes(key))
				.map(([key, value]) => {
					if (value instanceof RegExp) {
						return {
							pattern: value,
							alias: key
						};
					}

					if (Array.isArray(value)) {
						return value.map((v) => ({
							...v,
							alias: addAlias(key, v.alias)
						}));
					}

					return {
						...value,
						alias: addAlias(key, value.alias)
					};
				})
				.flat();
		}

		/**
		 * Builds a token pattern from an array of keywords and assigns an alias to it.
		 *
		 * @param {string[]} keywords Array of keywords
		 * @param {string|string[]} alias Alias to assign to every keyword in the pattern
		 * @returns {{pattern: RegExp, alias: string|string[]}}
		 */
		function getKeywordPattern(
			keywords: string[],
			alias: string | string[]
		): { pattern: RegExp; alias: string | string[] } {
			return {
				pattern: new RegExp(`(?<=\\s)(?:${keywords.join('|')})(?==)`),
				alias
			};
		}

		/**
		 * Builds a regex pattern that matches nested structures with a limited depth.
		 * We use it to match balanced delimiters, such as parentheses, brackets, or custom tokens (e.g., `$[...]` and `$(...)`).
		 *
		 * @param {string} openToken e.g. `$(` or `(` or `[`
		 * @param {string} closeToken e.g. `)` or `]`
		 * @param {object} options
		 * @param {number} [options.depth=3] maximum nesting depth
		 * @param {string} [options.flags=''] regex flags
		 * @param {boolean} [options.captureInner=false] whether to capture the open/close tokens or use lookarounds to get inner content only
		 * @returns {RegExp}
		 */
		function buildNestedRegex(
			openToken: string,
			closeToken: string,
			{
				depth = 3,
				flags = '',
				captureInner = false
			}: { depth?: number; flags?: string; captureInner?: boolean } = {}
		): RegExp {
			// Escape characters for regex patterns
			// @ts-expect-error any is fine here
			const escape = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

			// last char of openToken is the nesting char; prefix is anything before it (e.g. '$' from '$(')
			// eslint-disable-next-line prefer-const
			let [prefix, openerChar] = openToken;
			if (!openerChar) {
				// openToken is a single character string; prefix is empty
				prefix = '';
			} else {
				// openToken is a string with a prefix and a single character; remove the prefix from openToken
				openToken = openerChar;
			}

			prefix = escape(prefix);
			openToken = escape(openToken);
			closeToken = escape(closeToken);

			// Single non-special char (or escaped char, or line continuation)
			// Line continuation: backslash followed by optional \r and required \n
			const atom = `[^${openToken}${closeToken}\\\\]|${/\\./.source}|\\\\\\r?\\n`;

			// Build depth-limited pattern recursively:
			// content_0 = (?:atom)+
			// content_k = (?: (?:atom)+ | opener content_{k-1} closer )+
			let prev = `(?:${atom})+`; // content_0
			for (let level = 1; level <= depth; level++) {
				// Each unit is either atoms or a nested pair; require one or more units
				prev = `(?:(?:(?:${atom})+|(?:${openToken}${prev}${closeToken})))+`;
			}

			return new RegExp(
				captureInner
					? `(?<=${prefix}${openToken})${prev}(?=${closeToken})`
					: `${prefix}${openToken}${prev}${closeToken}`,
				flags
			);
		}

		// ============== End of helper functions ==============

		return {
			comment: {
				pattern: /(?<=^|\s)#.*$/m,
				greedy: true
			},

			// Note: Only arrays defined in one line are supported; they should always end with `;`!
			// Arrays appear in assignments or function calls, not as standalone scopes
			array: {
				// Match `{ ... }` that is NOT after `do=`, `else=`, and appears in array contexts
				// Arrays are typically: :local arr {1;2;3}, :put {1;2;3}, or {1;2;3},5
				// Scopes are: `do={...}`, `else={...}`, or standalone blocks
				// Exclude `do={` and `else={` by checking that we're not immediately after `do=` or `else=`
				pattern: /(?<!do=)(?<!else=)\{[^{}\r\n]*}(?=\s*[;=,)]|\s*$)/,
				greedy: true,
				inside: {
					punctuation: [
						{
							pattern: /^\{/,
							alias: 'begin-of-array'
						},
						{
							pattern: /}$/,
							alias: 'end-of-array'
						}
					],
					'array-item': [
						{
							// Array items with keys: "key"=value or key=value
							pattern: /(?<=\s|^)[^;=]+=[^;]+(?=;|$)/,
							inside: {
								key: {
									pattern: /(?:"[^"]+"|[^\s=][^=]*)(?==)/,
									inside: {
										punctuation: /^"|"$/
									}
								},
								value: [
									...getDataTypesPatterns(),
									// Fallback for values with unknown data type
									/(?<==)[^;]+/
								],
								operator: /=/
							}
						},
						// Array items without keys: number, boolean, etc.
						...getDataTypesPatterns().map((v) => {
							let source = v.pattern.source.replace(/^\^/, '');
							source = source.replace(/\(\?<=\\s\|=\|\^\)/, '(?<=\\s|=|^|\\{|;)');

							return {
								...v,
								// Add array item boundary: ensure it ends with ";" or it's the last item in the array
								pattern: new RegExp(`${source}(?=;|$)`, v.pattern.flags)
							};
						}),
						// TODO: Support strings

						// Fallback for values without keys and unknown data type
						/(?<=[\s{};]|^)[^;=]+(?=;|$)/
					],
					'array-item-delimiter': {
						pattern: /;/,
						alias: 'punctuation'
					}
				}
			},

			regex: {
				pattern: /~\s*"(?:\\.|[^"\\])*"/,
				greedy: true,
				inside: {
					operator: {
						pattern: /^~/,
						alias: 'regex-operator'
					},
					punctuation: /^"|"$/,
					$rest: 'regex'
				}
			},

			// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Variables
			variable: [
				// Definition
				{
					// Quoted variable after `:(global|local|set)`: `"var-name"`
					// Use (?:[^"\\]|\\.)* instead of [^"\\]*(?:\\.[^"\\]*)* to avoid exponential backtracking
					pattern: /(?<=:(?:global|local|set)\s+)"(?:[^"\\]|\\.)*"/,
					greedy: true
				},
				{
					// Unquoted variable after `:(global|local|set)`: `varName`
					pattern: /(?<=:(?:global|local|set)\s+)[A-Za-z\d]+/,
					greedy: true
				},

				// Reference
				...VARIABLE_PATTERNS
			],

			subexpression: {
				// Depth 3 nested parentheses
				pattern: buildNestedRegex('(', ')', {
					captureInner: true
				}),
				greedy: true,
				inside: 'mikrotik'
			},

			'command-substitution': {
				// Depth 3 nested square brackets
				pattern: buildNestedRegex('[', ']', {
					captureInner: true
				}),
				greedy: true,
				alias: 'command-concatenation',
				inside: 'mikrotik'
			},

			string: {
				pattern:
					/(?<!\$\s?)(?<!\b:(?:global|local|set)\s)(")(?:\\(?:\r\n|[\s\S])|(?!\1)[^\\\r\n])*\1/,
				greedy: true,
				inside: {
					// Expressions inside strings: `$[...]` and  `$(...)`
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-ConcatenationOperators
					expression: {
						pattern: new RegExp(
							buildNestedRegex('$(', ')').source + '|' + buildNestedRegex('$[', ']').source
						),
						inside: {
							'begin-of-expression': {
								pattern: /^\$[[(]/,
								alias: 'punctuation'
							},
							'end-of-expression': {
								pattern: /[\])]$/,
								alias: 'punctuation'
							},
							$rest: 'mikrotik'
						}
					},
					variable: VARIABLE_PATTERNS,
					'substitution-operator': {
						pattern: /\$$/,
						alias: 'operator'
					},
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-ConstantEscapeSequences
					'escape-sequence': {
						pattern: /\\(?:["\\nrt$_abfv]|[0-9A-F]{2})/,
						alias: 'char'
					}
				}
			},

			keyword: [
				// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Whitespacebetweentokens
				/\b(?:from|in|on-error|step|to)(?==)/,
				{
					// `else` could be part of the previous pattern, but we need to assign it a different alias;
					// and since it must follow by `=`, we can't make it part of the general keywords pattern below
					pattern: /\belse(?==)/,
					alias: 'control-flow'
				},
				{
					// Loops and conditional statements (https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Loopsandconditionalstatements)
					pattern: /\b(?:do|for|foreach|if|while)(?=[\s{=[(])/,
					alias: 'control-flow'
				},
				{
					// `once` keyword for monitoring commands (e.g., `/interface monitor-traffic ether1 once`)
					pattern: /\bonce\b/
				}
			],

			parameter: [
				// Known parameter categories - these are explicitly listed for better highlighting
				// and to provide semantic meaning. However, RouterOS has hundreds of parameters
				// across many menus, and it's impractical to list them all.
				getKeywordPattern(PRINT_PARAMETERS, ['print-parameter', 'property']),
				// Print parameters without = (e.g., `where`, `brief`, `detail`)
				{
					pattern: new RegExp(`(?<=\\s)(?:${PRINT_PARAMETERS.join('|')})(?!\\s*=)`),
					alias: ['print-parameter', 'property']
				},
				getKeywordPattern(ADD_COMMAND_PARAMETERS, ['add-parameter', 'property']),
				getKeywordPattern(COMMON_PROPERTIES, ['common-property', 'property']),
				getKeywordPattern(FIREWALL_PARAMETERS, ['firewall-parameter', 'property']),
				getKeywordPattern(NETWORK_PARAMETERS, ['network-parameter', 'property']),
				getKeywordPattern(INTERFACE_PARAMETERS, ['interface-parameter', 'property']),
				getKeywordPattern(DHCP_PARAMETERS, ['dhcp-parameter', 'property']),
				getKeywordPattern(TIMEOUT_PARAMETERS, ['timeout-parameter', 'property']),
				getKeywordPattern(QUEUE_PARAMETERS, ['queue-parameter', 'property']),
				getKeywordPattern(NTP_PARAMETERS, ['ntp-parameter', 'property']),
				getKeywordPattern(SERVICE_PARAMETERS, ['service-parameter', 'property']),
				getKeywordPattern(WIRELESS_PARAMETERS, ['wireless-parameter', 'property']),
				getKeywordPattern(BRIDGE_PARAMETERS, ['bridge-parameter', 'property']),
				getKeywordPattern(IPSEC_PARAMETERS, ['ipsec-parameter', 'property']),
				getKeywordPattern(OSPF_PARAMETERS, ['ospf-parameter', 'property']),
				getKeywordPattern(BGP_PARAMETERS, ['bgp-parameter', 'property']),
				getKeywordPattern(ROUTING_PARAMETERS, ['routing-parameter', 'property']),
				{
					// Fallback pattern: matches any RouterOS parameter name
					// RouterOS parameters follow kebab-case naming (e.g., 'src-address', 'connection-state')
					// This pattern ensures comprehensive coverage of all parameters, including those
					// not explicitly listed above. It matches words and hyphens followed by =,
					// which is the standard RouterOS parameter assignment syntax.
					// Examples: 'some-new-parameter=', 'vlan-id=', 'tx-power-mode='
					pattern: /(?<=\s)[a-z][\w-]+(?==)/i,
					alias: 'property'
				}
			],

			command: [
				{
					// Known menu/submenu paths (more specific, should come before generic pattern)
					// Matches after `/` or after whitespace (for submenu paths like `/ppp secret`)
					pattern: new RegExp('(?<=\\/|\\s)(?:' + MENU_PATHS.join('|') + ')\\b'),
					alias: ['menu-path', 'function']
				},
				{
					// Generic menu path pattern (catches any other menu paths)
					// Only matches after `/` to avoid false positives with parameters
					pattern: /(?<=\/)[a-z]\w*/,
					alias: ['path', 'function']
				},
				{
					pattern: new RegExp('(?<=:)(?:' + GLOBAL_COMMANDS.join('|') + ')\\b'),
					alias: ['global-command', 'builtin']
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Functions
					pattern: /(?<=:)return\b/,
					alias: ['function-return', 'builtin']
				},
				{
					pattern: new RegExp('\\b(?:' + MENU_SPECIFIC_COMMANDS.join('|') + ')\\b'),
					alias: ['menu-specific-command', 'common-command', 'function']
				}
			],

			prefix: {
				pattern: /[:/]$/,
				alias: 'punctuation'
			},

			'end-of-command': {
				pattern: /;(?=\s|$)/,
				alias: 'punctuation'
			},

			property: {
				// Property names (e.g., `contents` in `/file get ... contents`, `running` in `/interface get ether1 running`)
				// Matches property names anywhere (not followed by `=`)
				// Includes both PROPERTIES and COMMON_PROPERTIES
				pattern: new RegExp(
					'\\b(?:' + PROPERTIES.join('|') + '|' + COMMON_PROPERTIES.join('|') + ')(?!\\s*=)'
				)
			},

			'property-value': {
				// Common property values (e.g., `pppoe` in `buffer=pppoe`, `yes` in `disabled=yes`)
				// Matches known enum-like values when they appear after `=` or after comma in lists
				// Ensures complete values: not part of filenames (no dot/slash after) or other complex values
				// Supports comma-separated lists (e.g., `policy=read,write,test`)
				pattern: new RegExp('(?<=,|=)\\s*(?:' + PROPERTY_VALUES.join('|') + ')(?=$|[^./\\w-])'),
				alias: 'attr-value'
			},

			...DATA_TYPES,

			operator: [
				{
					pattern: /->/,
					alias: 'access-array-element-operator'
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-ArithmeticOperators
					// Note: `-` can be both unary (e.g., `-42`) and binary (e.g., `$var - 1`)
					pattern: /[%*/+-]/,
					alias: 'arithmetic-operator'
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-LogicalOperators
					pattern: /!|&&|\|\||\b(?:and|in|or)\b/,
					alias: 'logical-operator'
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-BitwiseOperators
					pattern: /<<|>>|[~|^&]/,
					alias: 'bitwise-operator'
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-RelationalOperators
					pattern: /[<>]=?|!=/,
					alias: 'relational-operator'
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-ConcatenationOperators
					// Matches dots and commas used for concatenation, avoiding false positives in filenames (e.g., `file.txt`)
					// Matches when NOT between two word characters (excludes `file.txt`) OR when adjacent to quotes/brackets/operators
					pattern: /\B[.,]\B|(?<=[")\]}$])[.,]|[.,](?=["[({$])/,
					alias: 'concatenation-operator'
				},
				{
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-OtherOperators
					// Matches parentheses used for grouping (e.g., `(4+5)`)
					// Note: `subexpression` uses `captureInner: true`, so parentheses are still available here
					pattern: /[()]/,
					alias: 'grouping-operator'
				},
				{
					// Substitution operator: matches $ followed by a variable
					// Examples: $"my-var", $myVar, $varName
					// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-OtherOperators
					pattern: /\$$/,
					alias: 'substitution-operator'
				},
				/=/
			],

			'line-joining': {
				// https://help.mikrotik.com/docs/spaces/ROS/pages/47579229/Scripting#Scripting-Linejoining
				pattern: /\\(?=\r?\n|$)/m,
				alias: 'punctuation'
			},

			'begin-of-scope': {
				pattern: /\{/,
				alias: 'punctuation'
			},

			'end-of-scope': {
				pattern: /}/,
				alias: 'punctuation'
			},

			punctuation: /[[\]"]/
		};
	}
};

const registerMikrotikLanguage = (prismInstance = Prism) => {
	if (!prismInstance.languages.mikrotik) {
		// Ensure regex support exists before wiring the MikroTik grammar
		prismInstance.languages.regex ??= {};
		// @ts-expect-error fine
		prismInstance.languages.mikrotik = mikrotik.grammar();
	}
};

registerMikrotikLanguage();

export { registerMikrotikLanguage };

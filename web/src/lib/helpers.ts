type ShortcutOptions = {
	shift?: boolean
	alt?: boolean
	cmd_or_ctrl?: boolean
}
const is_mac = navigator.userAgent.indexOf('Mac') !== -1

function check_modifiers(e: KeyboardEvent | MouseEvent, options: ShortcutOptions) {
	const target = {
		shift: options.shift || false,
		alt: options.alt || false,
		ctrl: (!is_mac && options.cmd_or_ctrl) || false,
		meta: (is_mac && options.cmd_or_ctrl) || false,
	}

	const pressed = {
		shift: !!e.shiftKey,
		alt: !!e.altKey,
		ctrl: !!e.ctrlKey,
		meta: !!e.metaKey,
	}

	const ignore_ctrl = is_mac && e instanceof MouseEvent

	return (
		pressed.shift === target.shift &&
		pressed.alt === target.alt &&
		(pressed.ctrl === target.ctrl || ignore_ctrl) &&
		pressed.meta === target.meta
	)
}

export function check_shortcut(e: KeyboardEvent, key: string, options: ShortcutOptions = {}) {
	if (e.key.toUpperCase() !== key.toUpperCase()) return false
	return check_modifiers(e, options)
}
export function check_mouse_shortcut(e: MouseEvent, options: ShortcutOptions = {}) {
	return check_modifiers(e, options)
}

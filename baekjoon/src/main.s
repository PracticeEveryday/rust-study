	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h36db2947f6ed5456E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17h5f01ad38d44b054cE
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17ha61c943fd4aad6b8E
	.globl	__ZN3std2rt10lang_start17ha61c943fd4aad6b8E
	.p2align	2
__ZN3std2rt10lang_start17ha61c943fd4aad6b8E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp]
	mov	x0, x2
	ldr	x2, [sp]
	str	x0, [sp, #8]
	mov	x4, x3
	ldr	x3, [sp, #8]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_1@PAGE
	add	x1, x1, l___unnamed_1@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17heb9ef1177a8102d0E
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hec05f7d2ba59f5d3E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h36db2947f6ed5456E
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hca648286bc58dba4E
	sturb	w0, [x29, #-1]
	ldurb	w0, [x29, #-1]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments6new_v117h5d246e7f1ff6cae3E:
	.cfi_startproc
	sub	sp, sp, #144
	.cfi_def_cfa_offset 144
	stp	x29, x30, [sp, #128]
	add	x29, sp, #128
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp]
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	str	x3, [sp, #32]
	subs	x8, x1, x3
	cset	w8, lo
	tbnz	w8, #0, LBB3_2
	b	LBB3_1
LBB3_1:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #32]
	add	x9, x9, #1
	subs	x8, x8, x9
	cset	w8, hi
	and	w8, w8, #0x1
	strb	w8, [sp, #47]
	b	LBB3_3
LBB3_2:
	mov	w8, #1
	strb	w8, [sp, #47]
	b	LBB3_3
LBB3_3:
	ldrb	w8, [sp, #47]
	tbnz	w8, #0, LBB3_5
	b	LBB3_4
LBB3_4:
	ldr	x8, [sp, #32]
	ldr	x9, [sp]
	ldr	x10, [sp, #24]
	ldr	x11, [sp, #16]
	ldr	x12, [sp, #8]
	stur	xzr, [x29, #-32]
	str	x12, [x9]
	str	x11, [x9, #8]
	ldur	x12, [x29, #-32]
	ldur	x11, [x29, #-24]
	str	x12, [x9, #32]
	str	x11, [x9, #40]
	str	x10, [x9, #16]
	str	x8, [x9, #24]
	.cfi_def_cfa wsp, 144
	ldp	x29, x30, [sp, #128]
	add	sp, sp, #144
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB3_5:
	.cfi_restore_state
	stur	xzr, [x29, #-16]
	add	x0, sp, #48
	adrp	x8, l___unnamed_2@PAGE
	add	x8, x8, l___unnamed_2@PAGEOFF
	str	x8, [sp, #48]
	mov	w8, #1
	str	x8, [sp, #56]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #80]
	str	x8, [sp, #88]
	adrp	x8, l___unnamed_3@PAGE
	add	x8, x8, l___unnamed_3@PAGEOFF
	str	x8, [sp, #64]
	str	xzr, [sp, #72]
	adrp	x1, l___unnamed_4@PAGE
	add	x1, x1, l___unnamed_4@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h3bbf9265d206434cE
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h595b859cae200536E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17h49894190586080a3E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h49894190586080a3E:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp0:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hec05f7d2ba59f5d3E
	str	w0, [sp, #12]
Ltmp1:
	b	LBB5_3
LBB5_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB5_2:
Ltmp2:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB5_1
LBB5_3:
	ldr	w0, [sp, #12]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h5f01ad38d44b054cE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h11c2e0b3ffb931e6E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hca648286bc58dba4E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main4main17hb9bf57f50a56af79E:
	.cfi_startproc
	sub	sp, sp, #288
	.cfi_def_cfa_offset 288
	stp	x28, x27, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	.cfi_remember_state
	mov	w8, #1
	str	w8, [sp, #88]
	ldr	w8, [sp, #88]
	adds	w8, w8, #1
	str	w8, [sp, #84]
	cset	w8, vs
	tbnz	w8, #0, LBB9_2
	b	LBB9_1
LBB9_1:
	ldr	w8, [sp, #84]
	str	w8, [sp, #88]
	ldr	w8, [sp, #88]
	adds	w8, w8, #2
	str	w8, [sp, #80]
	cset	w8, vs
	tbnz	w8, #0, LBB9_4
	b	LBB9_3
LBB9_2:
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_5@PAGE
	add	x2, x2, l___unnamed_5@PAGEOFF
	bl	__ZN4core9panicking5panic17h23f868a19cef4049E
LBB9_3:
	ldr	w8, [sp, #80]
	str	w8, [sp, #88]
	ldr	w8, [sp, #88]
	adds	w8, w8, #2
	str	w8, [sp, #76]
	cset	w8, vs
	tbnz	w8, #0, LBB9_6
	b	LBB9_5
LBB9_4:
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_6@PAGE
	add	x2, x2, l___unnamed_6@PAGEOFF
	bl	__ZN4core9panicking5panic17h23f868a19cef4049E
LBB9_5:
	ldr	w8, [sp, #76]
	str	w8, [sp, #88]
	ldr	w8, [sp, #88]
	adds	w8, w8, #2
	str	w8, [sp, #72]
	cset	w8, vs
	tbnz	w8, #0, LBB9_8
	b	LBB9_7
LBB9_6:
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_7@PAGE
	add	x2, x2, l___unnamed_7@PAGEOFF
	bl	__ZN4core9panicking5panic17h23f868a19cef4049E
LBB9_7:
	ldr	w8, [sp, #72]
	str	w8, [sp, #88]
	ldr	w8, [sp, #88]
	adds	w8, w8, #2
	str	w8, [sp, #68]
	cset	w8, vs
	tbnz	w8, #0, LBB9_10
	b	LBB9_9
LBB9_8:
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_8@PAGE
	add	x2, x2, l___unnamed_8@PAGEOFF
	bl	__ZN4core9panicking5panic17h23f868a19cef4049E
LBB9_9:
	ldr	w8, [sp, #68]
	add	x9, sp, #88
	str	x9, [sp]
	str	w8, [sp, #88]
	ldr	w0, [sp, #88]
	ldr	w1, [sp, #88]
	bl	__ZN4main7add_num17h2526100505014ec5E
	ldr	x8, [sp]
	add	x9, sp, #92
	str	x9, [sp, #16]
	str	w0, [sp, #92]
	stur	x8, [x29, #-32]
	adrp	x8, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hd45a0fa339175923E@GOTPAGE
	ldr	x8, [x8, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hd45a0fa339175923E@GOTPAGEOFF]
	str	x8, [sp, #24]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	sub	x2, x29, #128
	stur	x9, [x29, #-128]
	stur	x8, [x29, #-120]
	add	x8, sp, #96
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_9@PAGE
	add	x0, x0, l___unnamed_9@PAGEOFF
	str	x0, [sp, #32]
	mov	w9, #2
	mov	x1, x9
	str	x1, [sp, #40]
	mov	w9, #1
	mov	x3, x9
	str	x3, [sp, #48]
	bl	__ZN4core3fmt9Arguments6new_v117h5d246e7f1ff6cae3E
	ldr	x0, [sp, #8]
	bl	__ZN3std2io5stdio6_print17hb79efcbb9b8c1566E
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	ldr	x0, [sp, #32]
	ldr	x1, [sp, #40]
	ldr	x3, [sp, #48]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	ldur	x9, [x29, #-48]
	ldur	x8, [x29, #-40]
	sub	x2, x29, #64
	stur	x9, [x29, #-64]
	stur	x8, [x29, #-56]
	sub	x8, x29, #112
	str	x8, [sp, #56]
	bl	__ZN4core3fmt9Arguments6new_v117h5d246e7f1ff6cae3E
	ldr	x0, [sp, #56]
	bl	__ZN3std2io5stdio6_print17hb79efcbb9b8c1566E
	.cfi_def_cfa wsp, 288
	ldp	x29, x30, [sp, #272]
	ldp	x28, x27, [sp, #256]
	add	sp, sp, #288
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB9_10:
	.cfi_restore_state
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_10@PAGE
	add	x2, x2, l___unnamed_10@PAGEOFF
	bl	__ZN4core9panicking5panic17h23f868a19cef4049E
	.cfi_endproc

	.p2align	2
__ZN4main7add_num17h2526100505014ec5E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	adds	w8, w0, w1
	stur	w8, [x29, #-4]
	cset	w8, vs
	tbnz	w8, #0, LBB10_2
	b	LBB10_1
LBB10_1:
	ldur	w0, [x29, #-4]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB10_2:
	.cfi_restore_state
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_11@PAGE
	add	x2, x2, l___unnamed_11@PAGEOFF
	bl	__ZN4core9panicking5panic17h23f868a19cef4049E
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17hb9bf57f50a56af79E@PAGE
	add	x0, x0, __ZN4main4main17hb9bf57f50a56af79E@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17ha61c943fd4aad6b8E
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h11c2e0b3ffb931e6E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h595b859cae200536E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hec05f7d2ba59f5d3E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hec05f7d2ba59f5d3E

	.section	__TEXT,__const
l___unnamed_12:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_2:
	.quad	l___unnamed_12
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l___unnamed_3:
	.byte	0

l___unnamed_13:
	.ascii	"/rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_4:
	.quad	l___unnamed_13
	.asciz	"K\000\000\000\000\000\000\0005\001\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_14:
	.ascii	"main.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_5:
	.quad	l___unnamed_14
	.asciz	"\007\000\000\000\000\000\000\000d\002\000\000\t\000\000"

	.section	__TEXT,__const
	.p2align	4, 0x0
_str.0:
	.ascii	"attempt to add with overflow"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_6:
	.quad	l___unnamed_14
	.asciz	"\007\000\000\000\000\000\000\000e\002\000\000\t\000\000"

	.p2align	3, 0x0
l___unnamed_7:
	.quad	l___unnamed_14
	.asciz	"\007\000\000\000\000\000\000\000f\002\000\000\t\000\000"

	.p2align	3, 0x0
l___unnamed_8:
	.quad	l___unnamed_14
	.asciz	"\007\000\000\000\000\000\000\000g\002\000\000\t\000\000"

	.p2align	3, 0x0
l___unnamed_10:
	.quad	l___unnamed_14
	.asciz	"\007\000\000\000\000\000\000\000h\002\000\000\t\000\000"

	.section	__TEXT,__const
l___unnamed_15:
	.byte	10

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_9:
	.quad	l___unnamed_3
	.space	8
	.quad	l___unnamed_15
	.asciz	"\001\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_11:
	.quad	l___unnamed_14
	.asciz	"\007\000\000\000\000\000\000\000q\002\000\000\004\000\000"

.subsections_via_symbols

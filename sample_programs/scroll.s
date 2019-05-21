############################################################ -*- asm -*-
# COMP1521 18s2 -- Assignment 1 -- Scrolling Text!
# Scroll letters from a message in argv[1]
#
# Base code by Jashank Jeremy
# Tweaked by John Shepherd
# $Revision: 1.5 $
#
# Edit me with 8-column tabs!

# Requires:
#  - `all_chars', defined in chars.s

# Provides:
	.globl	main # :: int, [char *], [char *] -> int
	.globl	setUpDisplay # :: int, int -> void
	.globl	showDisplay # :: void -> void
	.globl	delay # :: int -> vovid
	.globl	isUpper # :: char -> int
	.globl	isLower # :: char -> int

	.globl	CHRSIZE
	.globl	NROWS
	.globl	NDCOLS
	.globl	MAXCHARS
	.globl	NSCOLS
	.globl	CLEAR


########################################################################
	.data

	# /!\ NOTE /!\
	# In C, the values of the symbols `CHRSIZE', `NROWS', `NDCOLS',
	# `NSCOLS', `MAXCHARS', and `CLEAR' would be substituted during
	# preprocessing.  SPIM does not have preprocessing facilities,
	# so instead we provide these values in the `.data' segment.

	# # of rows and columns in each big char
CHRSIZE:	.word	9
	# number of rows in all matrices
NROWS:		.word	9
	# number of columns in display matrix
NDCOLS:		.word	80
	# max length of input string
MAXCHARS:	.word	100
	# number of columns in bigString matrix
	# max length of buffer to hold big version
	# the +1 allows for one blank column between letters
NSCOLS:		.word	9000	# (NROWS * MAXCHARS * (CHRSIZE + 1))
        # ANSI escape sequence for 'clear-screen'
CLEAR:	.asciiz "\033[H\033[2J"
# CLEAR:	.asciiz "__showpage__\n" # for debugging

main__0:	.asciiz	"Usage: ./scroll String\n"
main__1:	.asciiz	"Only letters and spaces are allowed in the string!\n"
main__2:	.asciiz "String mush be < "
main__3:	.asciiz " chars\n"
main__4:	.asciiz "Please enter a string with at least one character!\n"

	.align	4
theString:	.space	101	# MAXCHARS + 1
	.align	4
display:	.space	720	# NROWS * NDCOLS
	.align	4
bigString:	.space	81000	# NROWS * NSCOLS

space_str: 
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '
	.byte ' ',' ',' ',' ',' ',' ',' ',' ',' '

########################################################################
# .TEXT <main>
	.text
main:

# Frame:	$fp, $ra, ...
# Uses:		$a0, $a1, $t0, $t1, $t2, $s0, $s1
# Clobbers:	...

# Locals:
#	- `theLength' in $s0
#	- `bigLength' in $s1
#	- `ch' in $s2
#	- `str' in $t2
#	- `i' in $s3
#	- `j' in $s4
#	- `row' in $s5
#	- `col' in $s6
#	- `iterations' in $s7
#	- `startingCol' in $s8

# Structure:
#	main
#	-> [prologue]
#	-> main_argc_gt_two
#	-> main_PTRs_init
#	  -> main_PTRs_cond
#	    -> main_ch_notspace
#	    -> main_ch_isLower
#	    -> main_ch_isSpace
#	  -> main_PTRs_step
#	-> main_PTRs_f
#	[theLength cond]
#	  | main_theLength_ge_MAXCHARS
#	  | main_theLength_lt_MAXCHARS
#	  | main_theLength_lt_1
#	  | main_theLength_ge_1
#	-> [epilogue]

# Code:
	# set up stack frame
	sw	$fp, -4($sp)
	la	$fp, -4($sp)
	sw	$ra, -4($fp)  # note: switch to $fp-relative
	sw	$s0, -8($fp)
	sw	$s1, -12($fp)
	sw	$s2, -16($fp)
	sw	$s3, -20($fp)
	sw	$s4, -24($fp)
	sw	$s5, -28($fp)
	sw	$s6, -32($fp)
	sw	$s7, -36($fp)
	sw	$s8, -40($fp)
	addi	$sp, $sp, -40

	# if (argc < 2)
	li	$t0, 2
	bge	$a0, $t0, main_argc_gt_twUpper
	nop	# in delay slot
	# printf(...)
	la	$a0, main__0
	li	$v0, 4 # PRINT_STRING_SYSCALL
	syscall
	# return 1  =>  load $v0, jump to epilogue
	li	$v0, 1
	j	main__post
	nop	# in delay slot
main_argc_gt_two:

	move	$s0, $zero
main_PTRs_init:
	# s = argv[1]
	lw	$t2, 4($a1)
main_PTRs_cond:
	# optimisation: `ch = *s' now
	# (ch = )*s
	lb	$s2, ($t2)
	# *s != '\0'  =>  ch != 0
	beqz	$s2, main_PTRs_f
	nop	# in delay slot

	# if (!isUpper(ch))
main_ch_upper:
	move	$a0, $s2
	jal	isUpper
	nop	# in delay slot
	beqz	$v0, main_ch_lower
	nop	# in delay slot
	j	main_ch_ok
	nop	# in delay slot
	# if (!isLower(ch))
main_ch_lower:
	move	$a0, $s2
	jal	isLower
	nop	# in delay slot
	beqz	$v0, main_ch_space
	nop	# in delay slot
	j	main_ch_ok
	nop	# in delay slot
	# if (ch != ' ')
main_ch_space:
	li	$t0, ' '
	bne	$s2, $t0, main_ch_fail
	nop	# in delay slot
	j	main_ch_ok
	nop	# in delay slot

main_ch_fail:
	# printf(...)
	la	$a0, main__1
	li	$v0, 4 # PRINT_STRING_SYSCALL
	syscall
	# exit(1)  =>  return 1  =>  load $v0, jump to epilogue
	li	$v0, 1
	j	main__post
	nop	# in delay slot

main_ch_ok:
	# if (theLength >= MAXCHARS)
	la	$t0, MAXCHARS
	lw	$t0, ($t0)
	# break  =>  jump out of for(*s...)
	bge	$s0, $t0, main_PTRs_f

	# theString[theLength]
	la	$t0, theString
	addu	$t0, $t0, $s0	# ADDU because address
	# theString[theLength] = ch
	sb	$s2, ($t0)

	# theLength++
	addi	$s0, $s0, 1

main_PTRs_step:
	# s++  =>  s = s + 1
	addiu	$t2, $t2, 1	# ADDIU because address
	j	main_PTRs_cond
	nop
main_PTRs_f:

	# theString[theLength] = ...
	la	$t0, theString
	addu	$t0, $t0, $s0	# ADDU because address
	# theString[theLength] = '\0'
	sb	$zero, ($t0)

	# CHRSIZE + 1
	la	$t0, CHRSIZE
	lw	$t0, ($t0)
	addi	$t0, $t0, 1
	# bigLength = theLength * (CHRSIZE + 1)
	mul	$s1, $t0, $s0

	# if (theLength >= MAXCHARS)
	la	$t0, MAXCHARS
	lw	$t0, ($t0)
	blt	$s0, $t0, main_theLength_lt_MAXCHARS
	nop	# in delay slot
main_theLength_ge_MAXCHARS:
	# printf(..., ..., ...)
	la	$a0, main__2
	li	$v0, 4 # PRINT_STRING_SYSCALL
	syscall
	move	$a0, $t0
	li	$v0, 1 # PRINT_INT_SYSCALL
	syscall
	la	$a0, main__3
	li	$v0, 4 # PRINT_STRING_SYSCALL
	syscall
	# return 1  =>  load $v0, jump to epilogue
	li	$v0, 1
	j	main__post
	nop	# in delay slot
main_theLength_lt_MAXCHARS:

	# if (theLength < 1)
	li	$t0, 1
	bge	$s0, $t0, main_theLength_ge_1
	nop	# in delay slot
main_theLength_lt_1:
	# printf(...)
	la	$a0, main__4
	li	$v0, 4 # PRINT_STRING_SYSCALL
	syscall
	# exit(1)  =>  return 1  =>  load $v0, jump to epilogue
	li	$v0, 1
	j	main__post
	nop	# in delay slot
main_theLength_ge_1:

# set display to ' '
        # $s2 = ' '
        li $s2, ' '
        
        # $s7 = NROWS * NDCOLS
        lw $s7, NROWS
        lw $s8, NDCOLS
        mul $s7, $s7, $s8
        
        # $s8 = &display
        la $s8, display

        display_loop:
            # $s7--
            addi $s7, $s7, -1
            addi $s8, $s8, 1

            # *($s7) = ' '
            sb $s2, ($s7)

            beqz $s7, end_display_loop

        end_display_loop:
            nop


# Create bigchars array
        per_theLength_loop:
            # for ($s3 = )
            beq $s3, $s0, end_theLength_loop
            
            la $t0, theString
            addi $s2, $s3, $t0

            lb $s2, ($s2)

            li $t0, ' '
            beq $s2, $t0, char_is_space

            move $a0, $s2
            jal is_upper
            li $t0, 1
            beq $v0, $t0, char_is_upper
            
            move $a0, $s2
            jal is_lower
            li $t0, 1
            beq $v0, $t0, char_is_lower

            ## program should never reach this point

            char_is_upper:
                move $s8, $s2
                addi $s8, -'A'
                lw $t0, CHRSIZE
                mul $s8, $s8, $t0
                mul $s8, $s8, $t0

                la $t0, all_chars
                addi $s8, $s8, $t0
                
                j end_char_is
           
            char_is_lower:
                move $s8, $s2
                addi $s8, -'a'
                addi $s8, 26
                lw $t0, CHRSIZE
                mul $s8, $s8, $t0
                mul $s8, $s8, $t0

                la $t0, all_chars
                addi $s8, $s8, $t0
                
                j end_char_is
            
            char_is_space:
                la $s8, space_str

                j end_char_is
            
            end_char_is:
                nop

#	- `theLength' in $s0
#	- `bigLength' in $s1
#	- `ch' in $s2
#	- `str' in $t2
#	- `i' in $s3
#	- `j' in $s4
#	- `row' in $s5
#	- `col' in $s6
#	- `iterations' in $s7
#       - `which` in $s8
#	- `startingCol' in $s8
            row_load_loop:
                lw $t0, ROWSIZE
                beq $t0, $
                col_load_loop:
                    
                        # TODO: load word in loop

                    j col_load_loop
                end_col_load:
                    nop
                j row_load_loop
            end_row_load:
                nop

            addi $s3, $s3, 1
            j per_theLength_loop
        end_theLength_loop:
            nop

	# return 0
	move	$v0, $zero
main__post:
	# tear down stack frame
	lw	$s8, -40($fp)
	lw	$s7, -36($fp)
	lw	$s6, -32($fp)
	lw	$s5, -28($fp)
	lw	$s4, -24($fp)
	lw	$s3, -20($fp)
	lw	$s2, -16($fp)
	lw	$s1, -12($fp)
	lw	$s0, -8($fp)
	lw	$ra, -4($fp)
	la	$sp, 4($fp)
	lw	$fp, ($fp)
	jr	$ra
	nop	# in delay slot

########################################################################
# .TEXT <setUpDisplay>
	.text
setUpDisplay:

# Frame:	$fp, $ra, ...
# Uses:		$a0, $a1, ...
# Clobbers:	...

# Locals:
#	- `row' in $...
#	- `out_col' in $...
#	- `in_col' in $...
#	- `first_col' in $...
#	- ...

# Structure:
#	setUpDisplay
#	-> [prologue]
#	-> ...
#	-> [epilogue]

# Code:
	# set up stack frame


    sw      $fp, -4($sp)
    la      $fp, -4($sp)
    sw      $ra, -4($fp)
    sw      $s0, -8($fp)
    sw      $s1, -12($fp)
    sw      $s2, -16($fp)
    sw      $s3, -20($fp)
    sw      $s4, -24($fp)
    sw      $s5, -28($fp)
    addi    $sp, $sp, -32

    ble, $a0, $zero, elif_starting_gt0
    if_starting_gt0:
        li $s1, 0
        neg $s3, $a0
        j endif_starting_gt0

    elif_starting_gt0:
        move $a0, $s1
        li $t0, NROWS
        mul $s0, $s0, $t0
        la $s4, DISPLAY
        
        for_sgt0:
            beqz $s1, end_for_sgt0
            for_inner_sgt0:
                beqz $s0, end_inner_sgt0
                li $t0, ' '
                sw $t0, ($s4)
                
                addi $s4, $s4, 1
                addi $s0, $s0, -1
                j for_inner_sgt0
            end_inner_sgt0:
                nop
            addi $s1, $s1, -1 
            j for_sgt0
        end_for_sgt0:
            nop
        move $s3, $zero

    endif_starting_gt0:
        nop

    la $s4, display
    la $s5, bigString

    # TODO: last for loop

    move $s2, $s3
    for_bigString:
        lw $t0, NDCOLS
        bgte $s1, $t0, end_for_bigstring
        bgte

    end_for_bigstring:

    addi    $sp, $sp, 32
    lw      $s5, -28($fp)
    lw      $s4, -24($fp)
    lw      $s3, -20($fp)
    lw      $s2, -16($fp)
    lw      $s1, -12($fp)
    lw      $s0, -8($fp)
    lw      $ra, -4($fp)
    lw      $sp, 4($fp)
    la      $sp, 4($fp)
    lw      $fp, ($fp)

    jr      $ra
	# tear down stack frame
	lw	$ra, -4($fp)
	la	$sp, 4($fp)
	lw	$fp, ($fp)
	jr	$ra
	nop	# in delay slot

########################################################################
# .TEXT <showDisplay>
	.text
showDisplay:
## KINDA DONE

# Frame:	$fp, $ra, ...
# Uses:		...
# Clobbers:	...

# Locals:
#	- `i' in $s0
#	- `j' in $s1
#       - `addr` in $s2
#	- ...

# Structure:
#	showDisplay
#	-> [prologue]
#	-> ...
#	-> [epilogue]

# Code:
	# set up stack frame
	sw	$fp, -4($sp)
	la	$fp, -4($sp)
	sw	$ra, -4($fp)
	la	$sp, -8($fp)
        sw      $s0, -12($fp)
        sw      $s1, -16($fp)
        sw      $s2, -20($fp)


        la $a0, CLEAR
        li $v0, 4
        syscall
            
        lw $s0, NROWS
        lw $s1, NDCOLS
        la $s2, display

        display_loop_begin:
            addi $s1, -1
            addi $s2, 1
            beqz $s0, display_loop_end

            li $v0, 11
            lb $a0, ($s2)
            syscall

            beqz $s1, end_row
            j end_row_end
            
            end_row:
                lw $s1, NDCOLS
                addi $s0, -1
            end_row_end:
                nop
            j display_loop_begin
        display_loop_end:
            nop


	# tear down stack frame
        lw      $s2, -20($fp)
        lw      $s1, -16($fp)
        lw      $s0, -12($fp)
	lw	$ra, -4($fp)
	la	$sp, 4($fp)
	lw	$fp, ($fp)
	jr	$ra
	nop	# in delay slot

########################################################################
# .TEXT <delay>
	.text
delay:

# Frame:	$fp, $ra
# Uses:		$a0, $t0, $t1, $t2, $t3, $t4, $t5
# Clobbers:	$t0, $t1, $t2, $t3, $t4, $t5

# Locals:
#	- `n' in $a0
#	- `x' in $t0
#	- `i' in $t1
#	- `j' in $t2
#	- `k' in $t3

# Structure:
#	delay
#	-> [prologue]
#	-> delay_i_init
#	-> delay_i_cond
#	   -> delay_j_init
#	   -> delay_j_cond
#	      -> delay_k_init
#	      -> delay_k_cond
#	         -> delay_k_step
#	      -> delay_k_f
#	      -> delay_j_step
#	   -> delay_j_f
#	   -> delay_i_step
#	-> delay_i_f
#	-> [epilogue]

# Code:
	sw	$fp, -4($sp)
	la	$fp, -4($sp)
	sw	$ra, -4($fp)
	la	$sp, -8($fp)

	# x <- 0
	move	$t0, $zero
	# These values control the busy-wait.
	li	$t4, 20000
	li	$t5, 1000

delay_i_init:
	# i = 0;
	move	$t1, $zero
delay_i_cond:
	# i < n;
	bge	$t1, $a0, delay_i_f
	nop	# in delay slot

delay_j_init:
	# j = 0;
	move	$t2, $zero
delay_j_cond:
	# j < DELAY_J;
	bge	$t2, $t4, delay_j_f
	nop	# in delay slot

delay_k_init:
	# k = 0;
	move	$t3, $zero
delay_k_cond:
	# k < DELAY_K;
	bge	$t3, $t5, delay_k_f
	nop	# in delay slot

	# x = x + 1
	addi	$t0, $t0, 1

delay_k_step:
	# k = k + 1
	addi	$t3, $t3, 1
	j	delay_k_cond
	nop	# in delay slot
delay_k_f:

delay_j_step:
	# j = j + 1
	addi	$t2, $t2, 1
	j	delay_j_cond
	nop	# in delay slot
delay_j_f:

delay_i_step:
	# i = i + 1
	addi	$t1, $t1, 1
	j	delay_i_cond
	nop	# in delay slot
delay_i_f:

delay__post:
	# tear down stack frame
	lw	$ra, -4($fp)
	la	$sp, 4($fp)
	lw	$fp, ($fp)
	jr	$ra
	nop	# in delay slot

########################################################################
# .TEXT <isUpper>
	.text
isUpper:

# Frame:	$fp, $ra
# Uses:		$a0
# Clobbers:	$v0

# Locals:
#	- `ch' in $a0
#	- ... $v0 used as temporary register

# Structure:
#	isUpper
#	-> [prologue]
#	[ch cond]
#	   | isUpper_ch_ge_a
#	   | isUpper_ch_le_z
#	   | isUpper_ch_lt_a
#	   | isUpper_ch_gt_z
#	-> isUpper_ch_phi
#	-> [epilogue]

# Code:
	# set up stack frame
	sw	$fp, -4($sp)
	la	$fp, -4($sp)
	sw	$ra, -4($fp)
	la	$sp, -8($fp)

	# if (ch >= 'A')
	li	$v0, 'A'
	blt	$a0, $v0, isUpper_ch_lt_a
	nop	# in delay slot
isUpper_ch_ge_a:
	# if (ch <= 'Z')
	li	$v0, 'Z'
	bgt	$a0, $v0, isUpper_ch_gt_z
	nop	# in delay slot
isUpper_ch_le_z:
	addi	$v0, $zero, 1
	j	isUpper_ch_phi
	nop	# in delay slot

	# ... else
isUpper_ch_lt_a:
isUpper_ch_gt_z:
	move	$v0, $zero
	# fallthrough
isUpper_ch_phi:

isUpper__post:
	# tear down stack frame
	lw	$ra, -4($fp)
	la	$sp, 4($fp)
	lw	$fp, ($fp)
	jr	$ra
	nop	# in delay slot

########################################################################
# .TEXT <isLower>
	.text
isLower:

# Frame:	$fp, $ra
# Uses:		$a0
# Clobbers:	$v0

# Locals:
#	- `ch' in $a0
#	- ... $v0 used as temporary register

# Structure:
#	isLower
#	-> [prologue]
#	[ch cond]
#	   | isLower_ch_ge_a
#	   | isLower_ch_le_z
#	   | isLower_ch_lt_a
#	   | isLower_ch_gt_z
#	-> isLower_ch_phi
#	-> [epilogue]

# Code:
	# set up stack frame
	sw	$fp, -4($sp)
	la	$fp, -4($sp)
	sw	$ra, -4($fp)
	la	$sp, -8($fp)

	# if (ch >= 'a')
	li	$v0, 'a'
	blt	$a0, $v0, isLower_ch_lt_a
	nop	# in delay slot
isLower_ch_ge_a:
	# if (ch <= 'z')
	li	$v0, 'z'
	bgt	$a0, $v0, isLower_ch_gt_z
	nop	# in delay slot
isLower_ch_le_z:
	addi	$v0, $zero, 1
	j	isLower_ch_phi
	nop	# in delay slot

	# ... else
isLower_ch_lt_a:
isLower_ch_gt_z:
	move	$v0, $zero
	# fallthrough
isLower_ch_phi:

isLower__post:
	# tear down stack frame
	lw	$ra, -4($fp)
	la	$sp, 4($fp)
	lw	$fp, ($fp)
	jr	$ra
	nop	# in delay slot

#################################################################### EOF

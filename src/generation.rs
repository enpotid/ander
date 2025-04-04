use crate::parser::{NodeBinExpr::*, NodeExpr::*, NodePExpr::*, NodeStmt::*, NodeTerm::*, *};
use std::{collections::HashMap, process::exit};

struct Var {
    stack_loc: usize,
}

pub fn gen_prog(prog: NodeProg) -> String {
    let mut output = String::from("section .text\n    global _start\n\n_start:\n");

    let mut msg_num = 0;
    let mut stack_size = 0;
    let mut datas = String::new();
    let mut vars: HashMap<String, Var> = HashMap::new();
    for stmt in prog.stmts {
        output.push_str(&gen_stmt(
            stmt,
            &mut stack_size,
            &mut msg_num,
            &mut vars,
            &mut datas,
        ));
    }

    output.push_str("    mov rax, 60\n    mov rdi, 0\n    syscall\n\n");
    output.push_str(
        "print_number:
    cmp rax, 0
    jne .compute
    mov rax, 1
    mov rdi, 1
    mov rsi, zero_msg
    mov rdx, zero_len
    syscall
    ret

.compute:
    xor r8, r8

.compute_loop:
    xor rdx, rdx
    mov rbx, 10
    div rbx
    add dl, '0'
    push rdx
    inc r8
    cmp rax, 0
    jne .compute_loop

.print_loop:
    pop rax
    sub rsp, 8
    mov byte [rsp], al
    mov rsi, rsp
    mov rax, 1
    mov rdi, 1
    mov rdx, 1
    syscall
    add rsp, 8
    dec r8
    cmp r8, 0
    jne .print_loop
    ret\n\n",
    );
    output.push_str("section .data\n");
    output.push_str(&datas);
    output.push_str("    zero_msg db '0'\n    zero_len equ $ - zero_msg");

    output
}

fn gen_term(term: &NodeTerm, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match term {
        IntLit(expr_int_lit) => format!(
            "    mov rax, {}\n{}",
            expr_int_lit.clone().int_lit.value.unwrap(),
            push("rax", stack_size)
        ),
        Ident(expr_ident) => {
            if !vars.contains_key(&expr_ident.ident.value.clone().unwrap()) {
                println!(
                    "undeclared identifier: {}",
                    expr_ident.clone().ident.value.unwrap()
                );
                exit(1);
            } else if *stack_size
                == vars
                    .get(&expr_ident.ident.value.clone().unwrap())
                    .unwrap()
                    .stack_loc
            {
                println!(
                    "it cannot reference itself: {}",
                    expr_ident.clone().ident.value.unwrap()
                );
                exit(1);
            }
            let var = vars.get(&expr_ident.ident.value.clone().unwrap()).unwrap();
            push(
                &format!("QWORD [rsp + {}]", (*stack_size - var.stack_loc - 1) * 8),
                stack_size,
            )
        }
        Paren(expr_paren) => gen_expr(expr_paren.clone().expr, stack_size, vars),
    }
}

fn gen_bin_expr(
    bin_expr: &NodeBinExpr,
    stack_size: &mut usize,
    vars: &mut HashMap<String, Var>,
) -> String {
    match bin_expr {
        Add(add) => format!(
            "{}{}{}{}    add rax, rbx\n{}",
            gen_expr(add.lhs.clone(), stack_size, vars),
            gen_expr(add.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Sub(sub) => format!(
            "{}{}{}{}    sub rax, rbx\n{}",
            gen_expr(sub.lhs.clone(), stack_size, vars),
            gen_expr(sub.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Multi(multi) => format!(
            "{}{}{}{}    mul rbx\n{}",
            gen_expr(multi.lhs.clone(), stack_size, vars),
            gen_expr(multi.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Div(div) => format!(
            "{}{}{}{}    xor rdx, rdx\n    div rbx\n{}",
            gen_expr(div.lhs.clone(), stack_size, vars),
            gen_expr(div.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Mod(modd) => format!(
            "{}{}{}{}    xor rdx, rdx\n    div rbx\n{}",
            gen_expr(modd.lhs.clone(), stack_size, vars),
            gen_expr(modd.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rdx", stack_size)
        ),
    }
}

fn gen_expr(expr: NodeExpr, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match expr {
        Term(expr_term) => gen_term(expr_term.as_ref(), stack_size, vars),
        BinExpr(expr_bin) => gen_bin_expr(expr_bin.as_ref(), stack_size, vars),
    }
}

fn gen_stmt(
    stmt: NodeStmt,
    stack_size: &mut usize,
    msg_num: &mut usize,
    vars: &mut HashMap<String, Var>,
    datas: &mut String,
) -> String {
    match stmt {
        Exit(stmt_exit) => format!(
            "{}    mov rax, 60\n{}    syscall\n",
            gen_expr(stmt_exit.expr, stack_size, vars),
            pop("rdi", stack_size)
        ),
        Let(stmt_let) => {
            if vars.contains_key(&stmt_let.ident.value.clone().unwrap()) {
                println!("identifier already used: {}", stmt_let.ident.value.unwrap());
                exit(1);
            }
            vars.insert(
                stmt_let.ident.value.unwrap(),
                Var {
                    stack_loc: *stack_size,
                },
            );
            gen_expr(stmt_let.expr, stack_size, vars)
        }
        Print(stmt_print_list) => {
            let mut result = String::new();
            for stmt_print in stmt_print_list.plist {
                match stmt_print {
                    Str(s) => {
                        *msg_num += 1;
                        datas.push_str(&format!(
                            "    msg{} db '{}'\n    len{} equ $ - msg{}\n",
                            msg_num, s, msg_num, msg_num
                        ));
                        result.push_str(&format!("    mov rax, 1\n    mov rdi, 1\n    mov rsi, msg{}\n    mov rdx, len{}\n    syscall\n", msg_num, msg_num));
                    }
                    Expr(expr) => {
                        result.push_str(&gen_expr(expr, stack_size, vars));
                        result.push_str(&pop("rax", stack_size));
                        result.push_str("    call print_number\n");
                    }
                }
            }
            result
        }
    }
}

fn push(reg: &str, stack_size: &mut usize) -> String {
    *stack_size += 1;
    format!("    push {reg}\n")
}

fn pop(reg: &str, stack_size: &mut usize) -> String {
    *stack_size -= 1;
    format!("    pop {reg}\n")
}

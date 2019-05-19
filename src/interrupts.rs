use crate::{println, gdt};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

/// Initialize and set up the `InterruptDescriptorTable`.
pub fn init_idt() {
    IDT.load();
}

/// Handle breakpoint exceptions by pretty-printing the current stack frame.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Handle double faults. This handler acts as a sort of "catch all" for any exception that
/// occurs for which a handler has not yet been written.
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64
) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    use crate::{serial_print, serial_println};

    #[test_case]
    fn test_breakpoint_exception() {
        serial_print!("test_breakpoint_exception... ");
        x86_64::instructions::interrupts::int3();
        serial_println!("[ok]");
    }
}

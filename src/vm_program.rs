/// Creates an enum with a public function `from_name` that returns the corresponding enum variant
/// given a matching string.
macro_rules! keyword_enum {
    ($EnumName:ident {
        $($EnumVariantName:ident $name_in_source:literal),*$(,)?
        $(($($extra_variant:tt)*) $($ev_match_arm:tt)*),*
    }) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum $EnumName {
            $($EnumVariantName,)*
            $($($extra_variant)*,)*
        }
        impl $EnumName {
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($name_in_source => Some(Self::$EnumVariantName),)*
                    $($($ev_match_arm)*,)*
                    _ => None,
                }
            }

            pub fn all_names() -> &'static [&'static str] {
                &[$($name_in_source,)*]
            }
        }
    }
}

// Everything on the left is the Rust identifier of an enum variant and the things on the right
// are the corresponding text of that keyword in the source file.
keyword_enum!(ArithmeticOpcode {
    Add "add",
    Sub "sub",
    Neg "neg",
    Eq "eq",
    Gt "gt",
    Lt "lt",
    And "and",
    Or "or",
    Not "not",
});

keyword_enum!(MemorySegment {
    Argument "argument",
    Local "local",
    Static "static",
    Constant "constant",
    This "this",
    That "that",
    Pointer "pointer",
    Temp "temp",
});

// Just different keywords used to identify non-arithmetic commands.
keyword_enum!(CommandName {
    Push "push",
    Pop "pop",
    Label "label",
    Goto "goto",
    IfGoto "if-goto",
    Function "function",
    Return "return",
    Call "call",
    (Arithmetic(ArithmeticOpcode))
        name => ArithmeticOpcode::from_name(name).map(|o| Self::Arithmetic(o))
});

#[derive(Clone, Copy, Debug)]
pub struct SymbolId(usize);

#[derive(Clone, Copy, Debug)]
pub enum VmCommand {
    Arithmetic(ArithmeticOpcode),
    Push(MemorySegment, usize),
    Pop(MemorySegment, usize),
    // Goto(SymbolId),
    // IfGoto(SymbolId),
    // Return,
}

#[derive(Debug)]
pub struct VmProgram {
    commands: Vec<VmCommand>,
}

impl VmProgram {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push_command(&mut self, command: VmCommand) {
        self.commands.push(command);
    }
}

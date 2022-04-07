use super::Eval;

impl Eval {
    pub(super) fn is_true(obj: &Box<dyn object::Object>) -> bool {
        match obj.object_type() {
            "Bool" => *obj.as_any().downcast_ref::<object::Bool>().unwrap().value,
            "Integer" => {
                let int_val = obj
                    .as_any()
                    .downcast_ref::<object::Integer>()
                    .unwrap()
                    .value;

                int_val != 0
            }
            "Null" => false,
            _ => true,
        }
    }

    pub(super) fn is_error(obj: &Box<dyn object::Object>) -> bool {
        obj.object_type() == "Error"
    }

    // issue #20
    // formatted string & variable argument using macro
    // format_argument! may helpful
    pub(super) fn new_error(value: String) -> Box<object::Error> {
        Box::new(object::Error { value })
    }
}

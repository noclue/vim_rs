use crate::vim_model::ManagedObject;
use crate::vim_model::VimModel;
use crate::printer::Printer;


pub struct ManagedObjectEmitter <'a>{
    mo: &'a ManagedObject,
    vim_model: &'a VimModel,
    printer: &'a mut dyn Printer,
}

impl <'a> ManagedObjectEmitter <'a> {
    pub fn new(mo: &'a ManagedObject, printer: &'a mut dyn Printer, vim_model: &'a VimModel) -> ManagedObjectEmitter<'a> {
        ManagedObjectEmitter {
            mo,
            vim_model,
            printer,
        }
    }

    pub fn emit(&mut self) -> Result<(), std::io::Error> {
        // self.emit_header()?;
        // self.emit_struct()?;
        // self.emit_impl()?;
        // self.emit_footer()?;
        Ok(())
    }

}
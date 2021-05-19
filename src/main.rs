use std::fmt::Debug;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nodo<T> {
    val: T,
    sig: Option<Box<Nodo<T>>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lista<T> {
    cabeza: Option<Box<Nodo<T>>>,
}
impl<T: Debug> Lista<T> {
    pub fn new() -> Self {
        Lista { cabeza: None }
    }
    pub fn append(&mut self, _elemento: T) {
        let sig: Option<Box<Nodo<T>>> = self.cabeza.take();
        let nuevo_nodo = Some(Box::new(Nodo {
            val: _elemento,
            sig: sig,
        }));
        self.cabeza = nuevo_nodo;
    }
    pub fn pop(&mut self) -> Option<T> {
        let cabeza: Option<Box<Nodo<T>>> = self.cabeza.take();
        cabeza.map(|x| {
            self.cabeza = x.sig;
            x.val
        })
    }
    pub fn imprimir_lista(self) {
        println!("{:?}", self);
    }
    pub fn len(&self) -> usize {
        match &self.cabeza {
            None => 0,
            Some(nodo) => {
                let mut actual: &Nodo<T> = nodo;
                let mut length: usize = 1;
                while actual.sig.is_some() {
                    actual = actual.sig.as_ref().unwrap();
                    length = length + 1;
                }
                length
            }
        }
    }
}

fn main() {
    //Ejemplos
    let mut biblioteca = Lista::new();
    biblioteca.append(31);
    biblioteca.append(32);
    biblioteca.append(33);
    biblioteca.pop();
    println!("{:?}", biblioteca.len());
    biblioteca.imprimir_lista();
}

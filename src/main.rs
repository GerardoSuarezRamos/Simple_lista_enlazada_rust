use std::fmt::Debug;

// Estructura que contendra la informacion neesaria para implementar la lista
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nodo<T> {
    val: T,
    sig: Option<Box<Nodo<T>>>,
}
// Estructura encargada de contener los diferentes nodos creados y enlazados dentro de si
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lista<T> {
    cabeza: Option<Box<Nodo<T>>>,
}
impl<T: Debug> Lista<T> {
    // Crea una nueva instancia de la lista, esta es genenerica e inicia siempre en nulo
    pub fn new() -> Self {
        Lista { cabeza: None }
    }

    // agrega un nuevo nodo a la lista
    pub fn append(&mut self, _elemento: T) {
        // Se captura el valor dentro de la cabeza de la lista actual
        let sig: Option<Box<Nodo<T>>> = self.cabeza.take();

        // Se crea el nuevo nodo a insertar dentro de la lista
        let nuevo_nodo = Some(Box::new(Nodo {
            val: _elemento,
            sig: sig,
        }));

        // se inserva el nuevo modulo dentro de la lista, con la cabeza anterior dentro de si
        self.cabeza = nuevo_nodo;
    }

    // elimina el primer elemento de la lista
    pub fn pop(&mut self) -> Option<T> {
        // Se captura el valor ingresado en la cabeza de la lista actual
        let cabeza: Option<Box<Nodo<T>>> = self.cabeza.take();

        //se itera sobre ese valor con un map, para regresar una nueva lista sin el primer elemento
        cabeza.map(|x| {
            self.cabeza = x.sig;
            x.val
        })
    }

    //Imprime la lista
    pub fn print_list(self) {
        println!("{:?}", self);
    }

    // Imprime el largo de la lista utilizando estructuras de control
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
    //instancia una nueva lista, la cual siempre es nula al principio
    let mut biblioteca = Lista::new();

    // agrega nuevos elementos a la lista
    biblioteca.append(31);
    biblioteca.append(32);
    biblioteca.append(33);

    // Elimina el primer elemento de la lista
    biblioteca.pop();

    //Imprime el largo de la lista
    println!("{:?}", biblioteca.len());

    // Imprime el contenido de la lista
    biblioteca.print_list();
}

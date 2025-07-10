pub mod solution { // modulo solution

    use std::cmp::Ordering;
    use std::fmt; // per il trait Display
    use std::hash::{Hash, Hasher};
    use std::ops::Add; // per il trait Add
    use std::ops::AddAssign; // per il trait AddAssign (+=)


    #[derive(Debug, Clone, Copy, PartialEq)] // tratti da implementare
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError {
        ImaginaryNotZero,
    }

    impl ComplexNumber {
        pub fn new(real: f64, imag: f64) -> ComplexNumber {
            Self {real, imag} // self è il costruttore
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn from_real(real: f64) -> ComplexNumber {
            Self {real, imag: 0.0}
        }

        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag) // restituisce una tupla con i valori reali e immaginari
        }

    }

    impl fmt::Display for ComplexNumber {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // implementazione del trait Display
            if self.imag >= 0.0 {
                write!(f, "{} + {}i", self.real, self.imag) // scrive su f
            } else {
                write!(f, "{} - {}i", self.real, -self.imag)
            }
        }
    }

    impl Add for ComplexNumber { // implementazione del trait Add
        type Output = Self; // il tipo di output è Self, cioè ComplexNumber

        fn add(self, other: Self) -> Self { // implementazione del metodo add, prende self e other dello stesso tipo
            Self::new(self.real + other.real, self.imag + other.imag)
        }

    }

    impl Add<f64> for ComplexNumber { // implementazione del trait Add per sommare un numero reale
        type Output = Self;

        fn add(self, other: f64) -> Self { // RHS è un numero reale (f64)
            Self::new(self.real + other, self.imag)
        }
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, rhs: Self) { // incrementa self con rhs (&mut self !)
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl<'a> Add<&'a ComplexNumber> for ComplexNumber { // implementazione generica con ciclo di vita 'a associato al riferimento &'a ComplexNumber
        type Output = Self;

        fn add(self, other: &'a ComplexNumber) -> Self {
            // 'a è il ciclo di vita del riferimento, self è il valore, rust usa 'a per
            // assicurarsi che il riferimento rimanga valido durante l'operazione e venga
            // deallocato quando non serve più
            Self::new(self.real + other.real, self.imag + other.imag)
        }
    }

    impl<'a, 'b> Add<&'b ComplexNumber> for &'a ComplexNumber {
        // Add viene implementato per il tipo riferimento a ComplexNumber con ciclo di vita 'a
        // a cui viene addizionato un riferimento a ComplexNumber con ciclo di vita 'b
        type Output = ComplexNumber;

        fn add(self, other: &'b ComplexNumber) -> ComplexNumber {
            ComplexNumber::new(self.real + other.real, self.imag + other.imag)
        }
    }

    impl Default for ComplexNumber {
        fn default() -> Self {
            Self { real: 0.0, imag: 0.0 } // implementazione del trait Default
        }
    }
/*
    impl Into<f64> for ComplexNumber { // converte un ComplexNumber (solo parte reale) in un f64
        fn into(self) -> f64 {
            if self.imag == 0.0 {
                self.real
            }
            else { // se la parte immaginaria non è 0 da panic
                panic!("Cannot convert ComplexNumber to f64, imaginary part is not zero")
            }

        }
    }
*/
    impl TryInto<f64> for ComplexNumber { // converte un ComplexNumber (solo parte reale) in un f64
        type Error = ComplexNumberError;
        fn try_into(self) -> Result<f64, Self::Error> { // restituisce un errore per parte immaginaria non 0
            if self.imag == 0.0 {
                Ok(self.real)
            }else {
                Err(ComplexNumberError::ImaginaryNotZero)
            }
        }
    }

    impl From<f64> for ComplexNumber { // converte un f64 in un ComplexNumber
        fn from(real: f64) -> ComplexNumber {
            ComplexNumber::new(real, 0.0)
        }
    }

    impl Eq for ComplexNumber {}  // comparison trait

    impl PartialOrd<Self> for ComplexNumber { // supertrait di Ord (per il sorting)
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.real == other.real && self.imag == other.imag {
                Some(Ordering::Equal)
            } else if self.real > other.real || (self.real == other.real && self.imag > other.imag) {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            } // implemento l'ordinamento parziale
        }
    }

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.real == other.real && self.imag == other.imag {
                Ordering::Equal
            } else if self.real > other.real || (self.real == other.real && self.imag > other.imag) {
                Ordering::Greater
            } else {
                Ordering::Less
            } // implemento l'ordinamento totale
        }
    }

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 { // restituisce un riferimento a un numero reale
            &self.real
        }
    }
    
    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Converte la parte reale in bit e li passa all'hasher
            self.real.to_bits().hash(state);
            // Converte la parte immaginaria in bit e li passa all'hasher
            self.imag.to_bits().hash(state);
        }
    }
}

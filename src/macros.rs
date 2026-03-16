/// Declarative macro for alt_<n> function generation
///
/// # Example
///
/// ```text
/// define_alt!(<name>, <n> A<n> a<n> (, ...));
/// ```
#[macro_export]
macro_rules! gen_alt {
  ($name:ident, $($idx:tt $type_param:ident $arg:ident),*) => {
    fn $name<
      Dest,
      $($type_param: Parser<Dest = Dest>),*
    >(
      $($arg: $type_param),*
    ) -> Alt<($($type_param),*)> {
      Alt {
        parser: ($($arg),*),
      }
    }
    impl<Dest, $($type_param),*> Parser for Alt<($($type_param,)*)>
    where
      $($type_param: Parser<Dest = Dest>),*
    {
      type Dest = Dest;
      fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        $(
          if let Ok(ok) = self.parser.$idx.parse(input) {
            return Ok(ok);
          }
        )*
        Err(())
      }
    }
  };
}

/// Declarative macro for all_<n> function generation
///
/// # Example
///
/// ```text
/// define_all!(<name>, <n> A<n> a<n> (, ...));
/// ```
#[macro_export]
macro_rules! gen_all {
  ($name:ident, $($idx:tt $type_param:ident $arg:ident),*) => {
    fn $name<
      $($type_param: Parser),*
    >(
      $($arg: $type_param),*
    ) -> All<($($type_param),*)>{
      All {
        parser: ($($arg),*)
      }
    }
    impl<$($type_param),*> Parser for All<($($type_param),*)>
    where
      $($type_param: Parser),*
    {
      type Dest = ($($type_param::Dest),*);
      fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()>{
        let remaining = input;
        $(
          let (remaining, $arg) = self.parser.$idx.parse(remaining)?;
        )*
        Ok((remaining, ($($arg),*)))
      }
    }
  };
}

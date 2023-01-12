use nom::error::ParseError;
use nom::{AsChar, Compare, IResult, InputLength, InputTake, InputTakeAtPosition, Parser};

pub fn space0_around<F: Parser<I, O, E>, I, O, E>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, _) = nom::character::complete::multispace0(input)?;
        let (input, o) = parser.parse(input)?;
        let (input, _) = nom::character::complete::multispace0(input)?;
        Ok((input, o))
    }
}

pub fn space1_before<F: Parser<I, O, E>, I, O, E>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, _) = nom::character::complete::multispace1(input)?;
        let (input, o) = parser.parse(input)?;
        Ok((input, o))
    }
}

pub fn space0_tag<T, Input, Error: ParseError<Input>>(
    tag: T,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input: InputTakeAtPosition + InputTake + Compare<T>,
    <Input as InputTakeAtPosition>::Item: AsChar + Clone,
    T: InputLength + Clone,
{
    move |input: Input| {
        let tag = tag.clone();
        let (input, _) = nom::character::complete::multispace0(input)?;
        let (input, o) = nom::bytes::complete::tag(tag)(input)?;
        Ok((input, o))
    }
}

pub fn space1_tag<T, Input, Error: ParseError<Input>>(
    tag: T,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input: InputTakeAtPosition + InputTake + Compare<T>,
    <Input as InputTakeAtPosition>::Item: AsChar + Clone,
    T: InputLength + Clone,
{
    move |input: Input| {
        let tag = tag.clone();
        let (input, _) = nom::character::complete::multispace1(input)?;
        let (input, o) = nom::bytes::complete::tag(tag)(input)?;
        Ok((input, o))
    }
}

pub fn space1_tags<const TAGS: usize, T, Input, Error: ParseError<Input>>(
    tags: [T; TAGS],
) -> impl Fn(Input) -> IResult<Input, [Input; TAGS], Error>
where
    Input: InputTakeAtPosition + InputTake + Compare<T>,
    <Input as InputTakeAtPosition>::Item: AsChar + Clone,
    T: InputLength + Clone,
{
    move |mut input: Input| {
        let tags = tags.clone();
        let mut output: [std::mem::MaybeUninit<Input>; TAGS] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..TAGS {
            let (t, o) = nom::bytes::complete::tag(tags[i].clone())(input)?;
            output[i] = std::mem::MaybeUninit::new(o);
            input = if i != TAGS - 1 {
                let (t, _) = nom::character::complete::multispace1(t)?;
                t
            } else {
                t
            }
        }
        Ok((input, unsafe {
            (&output as *const _ as *const [Input; TAGS]).read()
        }))
    }
}

pub fn space0_tags<const TAGS: usize, T, Input, Error: ParseError<Input>>(
    tags: [T; TAGS],
) -> impl Fn(Input) -> IResult<Input, [Input; TAGS], Error>
where
    Input: InputTakeAtPosition + InputTake + Compare<T>,
    <Input as InputTakeAtPosition>::Item: AsChar + Clone,
    T: InputLength + Clone,
{
    move |mut input: Input| {
        let tags = tags.clone();
        let mut output: [std::mem::MaybeUninit<Input>; TAGS] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..TAGS {
            let (t, o) = nom::bytes::complete::tag(tags[i].clone())(input)?;
            output[i] = std::mem::MaybeUninit::new(o);
            input = if i != TAGS - 1 {
                let (t, _) = nom::character::complete::multispace1(t)?;
                t
            } else {
                t
            }
        }
        Ok((input, unsafe {
            (&output as *const _ as *const [Input; TAGS]).read()
        }))
    }
}

pub fn space1_tags_no_case<const TAGS: usize, T, Input, Error: ParseError<Input>>(
    tags: [T; TAGS],
) -> impl Fn(Input) -> IResult<Input, [Input; TAGS], Error>
where
    Input: InputTakeAtPosition + InputTake + Compare<T>,
    <Input as InputTakeAtPosition>::Item: AsChar + Clone,
    T: InputLength + Clone,
{
    move |mut input: Input| {
        let tags = tags.clone();
        let mut output: [std::mem::MaybeUninit<Input>; TAGS] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..TAGS {
            let (t, o) = nom::bytes::complete::tag_no_case(tags[i].clone())(input)?;
            output[i] = std::mem::MaybeUninit::new(o);
            input = if i != TAGS - 1 {
                let (t, _) = nom::character::complete::multispace1(t)?;
                t
            } else {
                t
            }
        }
        Ok((input, unsafe {
            (&output as *const _ as *const [Input; TAGS]).read()
        }))
    }
}

pub fn space0_tags_no_case<const TAGS: usize, T, Input, Error: ParseError<Input>>(
    tags: [T; TAGS],
) -> impl Fn(Input) -> IResult<Input, [Input; TAGS], Error>
where
    Input: InputTakeAtPosition + InputTake + Compare<T>,
    <Input as InputTakeAtPosition>::Item: AsChar + Clone,
    T: InputLength + Clone,
{
    move |mut input: Input| {
        let tags = tags.clone();
        let mut output: [std::mem::MaybeUninit<Input>; TAGS] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..TAGS {
            let (t, o) = nom::bytes::complete::tag_no_case(tags[i].clone())(input)?;
            output[i] = std::mem::MaybeUninit::new(o);
            input = if i != TAGS - 1 {
                let (t, _) = nom::character::complete::multispace1(t)?;
                t
            } else {
                t
            }
        }
        Ok((input, unsafe {
            (&output as *const _ as *const [Input; TAGS]).read()
        }))
    }
}

pub fn angle_bracket<F0, F1, I, O0, O1, E>(
    mut parser_before: F0,
    mut parser_inner: F1,
) -> impl FnMut(I) -> IResult<I, (O0, O1), E>
where
    F0: Parser<I, O0, E>,
    F1: Parser<I, O1, E>,
    E: ParseError<I>,
    I: InputTakeAtPosition + InputTake + Compare<&'static str>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, o0) = parser_before.parse(input)?;
        let (input, _) = space0_tag("<")(input)?;
        let (input, _) = nom::character::complete::multispace0(input)?;
        let (input, o1) = parser_inner.parse(input)?;
        let (input, _) = space0_tag(">")(input)?;
        Ok((input, (o0, o1)))
    }
}

pub fn seperated<F0, F1, F2, I, O0, O1, O2, E>(
    mut parser_0: F0,
    mut parser_sep: F1,
    mut parser_1: F2,
) -> impl FnMut(I) -> IResult<I, (O0, O1, O2), E>
where
    F0: Parser<I, O0, E>,
    F1: Parser<I, O1, E>,
    F2: Parser<I, O2, E>,
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, o0) = parser_0.parse(input)?;
        let (input, _) = nom::character::complete::multispace0(input)?;
        let (input, o1) = parser_sep.parse(input)?;
        let (input, _) = nom::character::complete::multispace0(input)?;
        let (input, o2) = parser_1.parse(input)?;
        Ok((input, (o0, o1, o2)))
    }
}

pub fn space0_between<I, O, E: ParseError<I>, List: Space0Between<I, O, E>>(
    mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
    move |i: I| l.space0_between(i)
}

pub trait Space0Between<I, O, E> {
    fn space0_between(&mut self, input: I) -> IResult<I, O, E>;
}

impl<Input, Output, Error: ParseError<Input>, A: Parser<Input, Output, Error>>
    Space0Between<Input, Output, Error> for (A,)
{
    fn space0_between(&mut self, input: Input) -> IResult<Input, Output, Error> {
        self.0.parse(input)
    }
}

macro_rules! succ (
    (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
    (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
    (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
    (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
    (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
    (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
    (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
    (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
    (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
    (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
    (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
    (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
    (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
    (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
    (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
    (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
    (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
    (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
    (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
    (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
    (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
);

macro_rules! space0_between_trait(
  ($first_parser:ident $second_parser:ident $($id_parser: ident)+, $first_output:ident $second_output:ident $($id_output: ident)+, $first_value:ident $second_value:ident $($id_value: ident)+) => (
    space0_between_trait!(__impl $first_parser $second_parser; $($id_parser)+, $first_output $second_output; $($id_output)+, $first_value $second_value; $($id_value)+);
  );
  (__impl $($current_parser:ident)*; $head_parser:ident $($id_parser: ident)+, $($current_output:ident)*; $head_output:ident $($id_output: ident)+, $($current_value:ident)*; $head_value:ident $($id_value: ident)+) => (
    space0_between_trait_impl!($($current_parser)*, $($current_output)*, $($current_value)*);

    space0_between_trait!(__impl $($current_parser)* $head_parser; $($id_parser)+, $($current_output)* $head_output; $($id_output)+, $($current_value)* $head_value; $($id_value)+);
  );
  (__impl $($current_parser:ident)*; $head_parser:ident, $($current_output:ident)*; $head_output:ident, $($current_value:ident)*; $head_value:ident) => (
    space0_between_trait_impl!($($current_parser)*, $($current_output)*, $($current_value)*);
    space0_between_trait_impl!($($current_parser)* $head_parser, $($current_output)* $head_output, $($current_value)* $head_value);
  );
);

macro_rules! space0_between_trait_impl(
  ($($id_parser:ident)+, $($id_output:ident)+, $($id_value:ident)+) => (
    impl<
        Input,
        $($id_output),+,
        Error,
        $($id_parser: Parser<Input, $id_output, Error>),+,
    > Space0Between<Input, ( $($id_output),+ ), Error> for ( $($id_parser),+ )
    where
        Input: InputTakeAtPosition,
        <Input as InputTakeAtPosition>::Item: AsChar + Clone,
        Error: ParseError<Input>,
    {
      fn space0_between(&mut self, input: Input) -> IResult<Input, ( $($id_output),+ ), Error> {
          space0_between_trait_inner!(0, self, input, $($id_parser)+, $($id_value)+, $($id_value)+)
      }
    }
  );
);

macro_rules! space0_between_trait_inner(
    ($it:tt, $self:expr, $input:ident, $head:ident $($id:ident)+, $id_head:ident $($id_value:ident)+, $($id_value_return:ident)+) => {
        match $self.$it.parse($input) {
            Ok(($input, $id_head)) => {
                match nom::character::complete::multispace0($input) {
                    Ok(($input, _)) => {
                        succ!($it, space0_between_trait_inner!($self, $input, $($id)+, $($id_value)+, $($id_value_return)+))
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    };
    ($it:tt, $self:expr, $input:ident, $head:ident, $id_head:ident, $($id_value:ident)+) => {
        match $self.$it.parse($input) {
            Ok(($input, $id_head)) => {
                Ok(($input, ($($id_value),+,)))
            },
            Err(e) => Err(e),
        }
    };
);

space0_between_trait!(
    P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16 P17 P18 P19 P20 P21,
    O0 O1 O2 O3 O4 O5 O6 O7 O8 O9 O10 O11 O12 O13 O14 O15 O16 O17 O18 O19 O20 O21,
    a b c d e f g h i j k l m n o p q r s t u v
);

pub fn space1_between<I, O, E: ParseError<I>, List: Space1Between<I, O, E>>(
    mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
    move |i: I| l.space1_between(i)
}

pub trait Space1Between<I, O, E> {
    fn space1_between(&mut self, input: I) -> IResult<I, O, E>;
}

impl<Input, Output, Error: ParseError<Input>, A: Parser<Input, Output, Error>>
    Space1Between<Input, Output, Error> for (A,)
{
    fn space1_between(&mut self, input: Input) -> IResult<Input, Output, Error> {
        self.0.parse(input)
    }
}

macro_rules! space1_between_trait(
  ($first_parser:ident $second_parser:ident $($id_parser: ident)+, $first_output:ident $second_output:ident $($id_output: ident)+, $first_value:ident $second_value:ident $($id_value: ident)+) => (
    space1_between_trait!(__impl $first_parser $second_parser; $($id_parser)+, $first_output $second_output; $($id_output)+, $first_value $second_value; $($id_value)+);
  );
  (__impl $($current_parser:ident)*; $head_parser:ident $($id_parser: ident)+, $($current_output:ident)*; $head_output:ident $($id_output: ident)+, $($current_value:ident)*; $head_value:ident $($id_value: ident)+) => (
    space1_between_trait_impl!($($current_parser)*, $($current_output)*, $($current_value)*);

    space1_between_trait!(__impl $($current_parser)* $head_parser; $($id_parser)+, $($current_output)* $head_output; $($id_output)+, $($current_value)* $head_value; $($id_value)+);
  );
  (__impl $($current_parser:ident)*; $head_parser:ident, $($current_output:ident)*; $head_output:ident, $($current_value:ident)*; $head_value:ident) => (
    space1_between_trait_impl!($($current_parser)*, $($current_output)*, $($current_value)*);
    space1_between_trait_impl!($($current_parser)* $head_parser, $($current_output)* $head_output, $($current_value)* $head_value);
  );
);

macro_rules! space1_between_trait_impl(
  ($($id_parser:ident)+, $($id_output:ident)+, $($id_value:ident)+) => (
    impl<
        Input,
        $($id_output),+,
        Error,
        $($id_parser: Parser<Input, $id_output, Error>),+,
    > Space1Between<Input, ( $($id_output),+ ), Error> for ( $($id_parser),+ )
    where
        Input: InputTakeAtPosition,
        <Input as InputTakeAtPosition>::Item: AsChar + Clone,
        Error: ParseError<Input>,
    {
      fn space1_between(&mut self, input: Input) -> IResult<Input, ( $($id_output),+ ), Error> {
          space1_between_trait_inner!(0, self, input, $($id_parser)+, $($id_value)+, $($id_value)+)
      }
    }
  );
);

macro_rules! space1_between_trait_inner(
    ($it:tt, $self:expr, $input:ident, $head:ident $($id:ident)+, $id_head:ident $($id_value:ident)+, $($id_value_return:ident)+) => {
        match $self.$it.parse($input) {
            Ok(($input, $id_head)) => {
                match nom::character::complete::multispace1($input) {
                    Ok(($input, _)) => {
                        succ!($it, space1_between_trait_inner!($self, $input, $($id)+, $($id_value)+, $($id_value_return)+))
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    };
    ($it:tt, $self:expr, $input:ident, $head:ident, $id_head:ident, $($id_value:ident)+) => {
        match $self.$it.parse($input) {
            Ok(($input, $id_head)) => {
                Ok(($input, ($($id_value),+,)))
            },
            Err(e) => Err(e),
        }
    };
);

space1_between_trait!(
    P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16 P17 P18 P19 P20 P21,
    O0 O1 O2 O3 O4 O5 O6 O7 O8 O9 O10 O11 O12 O13 O14 O15 O16 O17 O18 O19 O20 O21,
    a b c d e f g h i j k l m n o p q r s t u v
);

macro_rules! limit_inc
{
	($value:expr, $n:expr) =>
	{
		if $value < $n
		{
			$value += 1;
		}
	};
}

macro_rules! limit_dec
{
	($value:expr, $n:expr) =>
	{
		if $value > $n
		{
			$value -= 1;
		}
	};
}

macro_rules! limit_dec_by
{
	($value:expr, $n:expr) =>
	{
		let v = $n;
		if $value > v
		{
			$value -= v;
		}
		else
		{
			$value = 0;
		}
	};
}

macro_rules! limit_inc_by
{
	($value:expr, $n:expr, $max:expr) =>
	{
		if $value + $n < $max
		{
			$value += $n;
		}
		else
		{
			$value = $max;
		}
	};
}

macro_rules! swap
{
	($a:expr, $b:expr) =>
	{
		let c = $a;
		$a = $b;
		$b = c;
	};
}

BEGIN
{
	printf("tracing register shrinker\n");
}

kprobe:register_shrinker
{
	@[kstack] = count();
}

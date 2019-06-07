BEGIN
{
	printf("tracing page frees");
}

kprobe:shrink_slab
{
	@[kstack] = count();
}

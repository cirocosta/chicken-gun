// kprobe:shrink_slab
// {
// 	@[kstack] = count();
// }


kprobe:shrink_slab { 
       @start[tid] = nsecs; 
} 


kretprobe:shrink_slab /@start[tid]/ { 
	@ns[comm] = hist(nsecs - @start[tid]); 
	delete(@start[tid]); 
}


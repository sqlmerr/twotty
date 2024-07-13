import { cn } from '@/lib/utils';
import { Loader2 } from 'lucide-react';

const Loader = ({ className }: { className?: string }) => {
  return (
    <Loader2
      className={cn('text-primary/60 animate-spin flex h-screen justify-center items-center m-auto', className)}
    />
  );
};

export default Loader;
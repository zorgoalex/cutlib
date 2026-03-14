using System.Threading;

public sealed class RequestConcurrencyGate
{
    private readonly SemaphoreSlim _semaphore;

    public RequestConcurrencyGate(IConfiguration configuration)
    {
        int maxConcurrency = configuration.GetValue<int?>("LibCut:MaxConcurrentOptimizations") ?? 1;
        if (maxConcurrency <= 0)
            maxConcurrency = 1;

        _semaphore = new SemaphoreSlim(maxConcurrency, maxConcurrency);
    }

    public async ValueTask<IDisposable> EnterAsync(CancellationToken cancellationToken)
    {
        await _semaphore.WaitAsync(cancellationToken);
        return new Releaser(_semaphore);
    }

    private sealed class Releaser : IDisposable
    {
        private readonly SemaphoreSlim _semaphore;
        private bool _disposed;

        public Releaser(SemaphoreSlim semaphore)
        {
            _semaphore = semaphore;
        }

        public void Dispose()
        {
            if (_disposed)
                return;

            _disposed = true;
            _semaphore.Release();
        }
    }
}


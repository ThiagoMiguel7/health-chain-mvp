export type PromiseWithResolversProps<T> = {
  promise: Promise<T>;
  resolve: (value: T) => void;
  reject: (reason?: T) => void;
};

export function withResolvers<T>(): PromiseWithResolversProps<T> {
  let resolveFn: (value: T) => void;
  let rejectFn: (reason?: T) => void;

  const promiseWithResolvers: PromiseWithResolversProps<T> = {
    promise: new Promise<T>((resolve, reject) => {
      resolveFn = resolve;
      rejectFn = reject;
    }),
    resolve: resolveFn!,
    reject: rejectFn!,
  };

  return promiseWithResolvers;
}

/**
 * Awaitable function that waits a certain amount of time before resolving.
 * Useful for slowing down execution for debugging purposes.
 *
 * @param {number} time - time in milliseconds, defaults to 500 (500 milliseconds)
 */
export const delay = async (time: number = 500): Promise<void> =>
  new Promise(resolve => setTimeout(resolve, time));

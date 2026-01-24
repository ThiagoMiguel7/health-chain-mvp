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

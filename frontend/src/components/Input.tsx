export type InputProps = {
  title: string;
  value: string;
  placeholder?: string;
  children?: React.ReactNode;

  onChange: (value: string) => void;
};

export function Input({
  title,
  value,
  children,
  placeholder = '5Grw...',

  onChange,
}: Readonly<InputProps>): JSX.Element {
  const handleOnChange = ({
    target,
  }: React.ChangeEvent<HTMLInputElement>): void => onChange(target.value);

  return (
    <div>
      <label className='block text-sm font-medium text-gray-700 mb-2'>
        {title}
      </label>

      <input
        type='text'
        value={value}
        placeholder={placeholder}
        onChange={handleOnChange}
        className='w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all'
      />
      {children}
    </div>
  );
}

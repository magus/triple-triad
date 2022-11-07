type Props = {
  children?: React.ReactNode;
  className?: string;
  onClick?(): void;
  color?: keyof typeof Color;
};

export function Button(props: Props) {
  const color_className = get_color_className(props);
  const className = `text-4xl rounded border-b-4  py-4 px-8 font-bold ${color_className} ${props.className}`;

  return (
    <button className={className} onClick={props.onClick}>
      {props.children}
    </button>
  );
}

function get_color_className(props: Props) {
  switch (props.color) {
    case Color.slate:
      return 'border-slate-700 bg-slate-500 text-white hover:border-slate-500 hover:bg-slate-400';
    case Color.gray:
      return 'border-gray-700 bg-gray-500 text-white hover:border-gray-500 hover:bg-gray-400';
    case Color.zinc:
      return 'border-zinc-700 bg-zinc-500 text-white hover:border-zinc-500 hover:bg-zinc-400';
    case Color.neutral:
      return 'border-neutral-700 bg-neutral-500 text-white hover:border-neutral-500 hover:bg-neutral-400';
    case Color.stone:
      return 'border-stone-700 bg-stone-500 text-white hover:border-stone-500 hover:bg-stone-400';
    case Color.red:
      return 'border-red-700 bg-red-500 text-white hover:border-red-500 hover:bg-red-400';
    case Color.orange:
      return 'border-orange-700 bg-orange-500 text-white hover:border-orange-500 hover:bg-orange-400';
    case Color.amber:
      return 'border-amber-700 bg-amber-500 text-white hover:border-amber-500 hover:bg-amber-400';
    case Color.yellow:
      return 'border-yellow-700 bg-yellow-500 text-white hover:border-yellow-500 hover:bg-yellow-400';
    case Color.lime:
      return 'border-lime-700 bg-lime-500 text-white hover:border-lime-500 hover:bg-lime-400';
    case Color.green:
      return 'border-green-700 bg-green-500 text-white hover:border-green-500 hover:bg-green-400';
    case Color.emerald:
      return 'border-emerald-700 bg-emerald-500 text-white hover:border-emerald-500 hover:bg-emerald-400';
    case Color.teal:
      return 'border-teal-700 bg-teal-500 text-white hover:border-teal-500 hover:bg-teal-400';
    case Color.cyan:
      return 'border-cyan-700 bg-cyan-500 text-white hover:border-cyan-500 hover:bg-cyan-400';
    case Color.sky:
      return 'border-sky-700 bg-sky-500 text-white hover:border-sky-500 hover:bg-sky-400';
    case Color.blue:
      return 'border-blue-700 bg-blue-500 text-white hover:border-blue-500 hover:bg-blue-400';
    case Color.violet:
      return 'border-violet-700 bg-violet-500 text-white hover:border-violet-500 hover:bg-violet-400';
    case Color.purple:
      return 'border-purple-700 bg-purple-500 text-white hover:border-purple-500 hover:bg-purple-400';
    case Color.fuchsia:
      return 'border-fuchsia-700 bg-fuchsia-500 text-white hover:border-fuchsia-500 hover:bg-fuchsia-400';
    case Color.pink:
      return 'border-pink-700 bg-pink-500 text-white hover:border-pink-500 hover:bg-pink-400';
    case Color.rose:
      return 'border-rose-700 bg-rose-500 text-white hover:border-rose-500 hover:bg-rose-400';
    case Color.indigo:
    default:
      return 'border-indigo-700 bg-indigo-500 text-white hover:border-indigo-500 hover:bg-indigo-400';
  }
}

enum Color {
  slate = 'slate',
  gray = 'gray',
  zinc = 'zinc',
  neutral = 'neutral',
  stone = 'stone',
  red = 'red',
  orange = 'orange',
  amber = 'amber',
  yellow = 'yellow',
  lime = 'lime',
  green = 'green',
  emerald = 'emerald',
  teal = 'teal',
  cyan = 'cyan',
  sky = 'sky',
  blue = 'blue',
  indigo = 'indigo',
  violet = 'violet',
  purple = 'purple',
  fuchsia = 'fuchsia',
  pink = 'pink',
  rose = 'rose',
}

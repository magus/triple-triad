import { Card } from 'src/components/Card';

export function PlayerHand() {
  return (
    <div className="flex flex-col items-center">
      <div className="flex flex-row">
        <Card id="88" player />
        <div className="ml-8" />
        <Card id="75" player />
        <div className="ml-8" />
        <Card id="89" player />
      </div>
      <div className="mt-8" />
      <div className="flex flex-row">
        <Card id="93" player />
        <div className="ml-8" />
        <Card id="96" player />
      </div>
    </div>
  );
}

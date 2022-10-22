import * as shell from "child_process";

export function cli(command) {
  let output = shell.execSync(command).toString();
  output = output.trim();
  return output;
}

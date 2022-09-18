let names = [];

for await(let file of Deno.readDir("./assets")) {
  let name: string;

  if(parseInt(file.name[0])) {
    name = file.name.slice(4, file.name.length);
  } else {
    name = file.name;
  }

  names.push(name.replace(/\+/g, " ").slice(0, name.indexOf(".")));
}

names.sort();

Deno.writeTextFileSync("../frontend/public/exercises.json", JSON.stringify({ names: names }))

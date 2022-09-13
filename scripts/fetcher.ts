let groups = ["legs", "calves", "shoulders", "biceps", "triceps", "abs", "forearms", "back", "chest"];
let url = Deno.args[0];


for(let group of groups) {
  let result = await fetch(`http://${url}-${group}`);

  let content = await result.text();

  let urls = content
    .split("\n")
    .filter(x => x.includes("squarespace-cdn.com/content/v1"))
    .map(x => x.slice(x.indexOf('"'), x.length))
    .map(x => x.slice(1, x.length))
    .map(x => x.slice(0, x.indexOf('"'))).filter(x => x.startsWith("https://"));


  let indexes = urls
    .map(x => x.split("").reverse().join(""))
    .map(x => x.indexOf('/'));


  for(let i = 0; i < urls.length; i++) {
    let url = urls[i];
    let index = indexes[i];

    let exerciseName = url.slice(url.length - index, url.length).replace(".jpg", "").replace(".png", "") + ".png";

    let p = Deno.run({
      cmd: ["curl", "--output", `assets/${exerciseName}`, url],
    });

    await p.status();

  }
}

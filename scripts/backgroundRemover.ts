for await(let file of Deno.readDir("assets")) {
  let p = Deno.run({
    cmd: ["magick",
      `assets/${file.name}`,
      `-fuzz`, `2%`,
      `-fill`,
      `none`,
      `-draw`,
      "alpha 0,0 floodfill",
      `-channel`,
      `alpha`,
      `-blur`,
      `0x2`,
      `-level`,
      `50x100%`,
      `+channel`,
      `assets/${file.name}`
    ]
  });

  await p.status();

  console.log(file.name);
}

document.getElementById("btn")?.addEventListener("click", () => {
  fetch("/press", { method: "POST" });
});

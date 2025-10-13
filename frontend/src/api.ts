export async function fetchTodos() {
  const res = await fetch("http://localhost:3000/todos");
  return res.json();
}

export async function createTodo(title: string) {
  await fetch("http://localhost:3000/todos", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ title }),
  });
}

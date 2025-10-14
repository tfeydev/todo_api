import { useEffect, useState } from "react";

interface Todo {
  id: number;
  title: string;
  done: boolean;
}

// --- API Helpers ---
async function fetchTodos(): Promise<Todo[]> {
  const res = await fetch("http://localhost:3000/todos");
  if (!res.ok) throw new Error("Failed to fetch todos");
  return res.json();
}

async function createTodo(title: string) {
  await fetch("http://localhost:3000/todos", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ title }),
  });
}

async function updateTodo(id: number, title: string, done: boolean) {
  await fetch(`http://localhost:3000/todos/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ title, done }),
  });
}

async function deleteTodo(id: number) {
  await fetch(`http://localhost:3000/todos/${id}`, { method: "DELETE" });
}

// --- App Component ---
function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTitle, setNewTitle] = useState("");
  
  // Initial Dark Mode State synchron
  const getInitialDark = () => {
    const saved = localStorage.getItem("theme");
    if (saved === "dark") return true;
    if (saved === "light") return false;
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  };
  const [isDark, setIsDark] = useState(getInitialDark);

  const [editingId, setEditingId] = useState<number | null>(null);
  const [editTitle, setEditTitle] = useState("");

  // Load Todos & Apply Dark Mode
  useEffect(() => {
    fetchTodos().then(setTodos).catch(console.error);
  }, []);

  // Dark Mode effect
  useEffect(() => {
    document.documentElement.classList.toggle("dark", isDark);
    localStorage.setItem("theme", isDark ? "dark" : "light");
  }, [isDark]);

  const refreshTodos = async () => {
    const updated = await fetchTodos();
    setTodos(updated);
  };

  const handleAdd = async () => {
    if (!newTitle.trim()) return;
    await createTodo(newTitle);
    setNewTitle("");
    await refreshTodos();
  };

  const handleToggleDone = async (todo: Todo) => {
    await updateTodo(todo.id, todo.title, !todo.done);
    await refreshTodos();
  };

  const handleStartEdit = (todo: Todo) => {
    setEditingId(todo.id);
    setEditTitle(todo.title);
  };

  const handleSaveEdit = async (todo: Todo) => {
    if (!editTitle.trim()) return;
    await updateTodo(todo.id, editTitle, todo.done);
    setEditingId(null);
    setEditTitle("");
    await refreshTodos();
  };

  const handleDelete = async (id: number) => {
    await deleteTodo(id);
    await refreshTodos();
  };

  return (
    <div className="min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 font-sans px-4">
      <div className="max-w-xl mx-auto pt-10">
        {/* Header */}
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">Todo List</h1>
          <button
            onClick={() => setIsDark((prev) => !prev)}
            className="px-3 py-1 rounded border dark:border-gray-600"
          >
            {isDark ? "☀️ Light" : "🌙 Dark"}
          </button>
        </div>

        {/* New Todo */}
        <div className="flex gap-2 mb-8">
          <input
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
            placeholder="New task"
            className="flex-1 border px-3 py-2 rounded 
                       bg-white dark:bg-gray-800 dark:text-white 
                       border-gray-300 dark:border-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={handleAdd}
            className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
            disabled={!newTitle.trim()}
          >
            Add
          </button>
        </div>

        {/* Todo List */}
        <ul className="space-y-3">
          {todos.map((todo) => (
            <li 
              key={todo.id} 
              className="flex items-center justify-between p-3 rounded-lg 
                         bg-gray-50 dark:bg-gray-800 shadow-sm"
            >
              {editingId === todo.id ? (
                <div className="flex flex-1 gap-2 items-center">
                  <input
                    value={editTitle}
                    onChange={(e) => setEditTitle(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") handleSaveEdit(todo);
                      if (e.key === "Escape") setEditingId(null);
                    }}
                    autoFocus
                    className="flex-1 border px-2 py-1 rounded 
                               bg-white dark:bg-gray-700 dark:text-white"
                  />
                  <button 
                    onClick={() => handleSaveEdit(todo)}
                    className="text-sm px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700"
                  >
                    Save
                  </button>
                  <button 
                    onClick={() => setEditingId(null)}
                    className="text-sm px-3 py-1 border border-gray-400 dark:border-gray-600 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
                  >
                    Cancel
                  </button>
                </div>
              ) : (
                <div className="flex flex-1 items-center gap-3">
                  <input
                    type="checkbox"
                    checked={todo.done}
                    onChange={() => handleToggleDone(todo)}
                    className="h-5 w-5 rounded form-checkbox text-blue-600 dark:bg-gray-700"
                  />
                  <span 
                    className={`flex-1 ${todo.done ? 'line-through text-gray-500 dark:text-gray-400' : ''}`}
                    onDoubleClick={() => handleStartEdit(todo)}
                  >
                    {todo.title}
                  </span>
                  <button
                    onClick={() => handleStartEdit(todo)}
                    className="text-sm text-blue-500 hover:text-blue-700 dark:hover:text-blue-400 p-1"
                    title="Edit"
                  >
                    ✏️
                  </button>
                  <button
                    onClick={() => handleDelete(todo.id)}
                    className="text-sm text-red-500 hover:text-red-700 dark:hover:text-red-400 p-1 ml-1"
                    title="Delete"
                  >
                    🗑️
                  </button>
                </div>
              )}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;

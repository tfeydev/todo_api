import React, { useState, useEffect, useCallback } from "react";
import { useAuth } from './AuthContext';

// --- Types ---
interface Todo {
  id: number;
  title: string;
  done: boolean;
  score: number;
}

// --- API Helpers ---
async function fetchTodos(getHeaders: () => HeadersInit): Promise<Todo[]> {
  const res = await fetch("http://localhost:3000/todos", {
    headers: getHeaders(),
  });
  if (res.status === 401) throw new Error("Unauthorized");
  if (!res.ok) throw new Error("Failed to fetch todos");
  return res.json();
}

async function createTodo(getHeaders: () => HeadersInit, title: string) {
  const res = await fetch("http://localhost:3000/todos", {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ title }),
  });
  if (res.status === 401) throw new Error("Unauthorized");
  if (!res.ok) throw new Error("Failed to create todo");
}

async function updateTodo(getHeaders: () => HeadersInit, id: number, title: string, done: boolean) {
  const res = await fetch(`http://localhost:3000/todos/${id}`, {
    method: "PUT",
    headers: getHeaders(),
    body: JSON.stringify({ title, done }),
  });
  if (res.status === 401) throw new Error("Unauthorized");
  if (!res.ok) throw new Error("Failed to update todo");
}

async function deleteTodo(getHeaders: () => HeadersInit, id: number) {
  const res = await fetch(`http://localhost:3000/todos/${id}`, { 
    method: "DELETE",
    headers: getHeaders(),
  });
  if (res.status === 401) throw new Error("Unauthorized");
  if (!res.ok && res.status !== 204) throw new Error("Failed to delete todo");
}

// --- Component ---
const TodoList: React.FC<{ isDark: boolean, toggleDark: () => void }> = ({ isDark, toggleDark }) => {
    // WICHTIG: isReady hinzugefügt!
    const { logout, getAuthHeaders, token, isReady } = useAuth();

    // Todo State
    const [todos, setTodos] = useState<Todo[]>([]);
    const [newTitle, setNewTitle] = useState("");
    const [editingId, setEditingId] = useState<number | null>(null);
    const [editTitle, setEditTitle] = useState("");

    // --- Data Management ---
    const refreshTodos = useCallback(async () => {
        if (!token) return;

        try {
            const updated = await fetchTodos(getAuthHeaders);
            setTodos(updated);
        } catch (e: any) {
            console.error("Error refreshing todos:", e.message);
            if (e.message === "Unauthorized") {
                logout();
            }
        }
    }, [getAuthHeaders, logout, token]);

    useEffect(() => {
        // WICHTIG: Warte auf isReady UND token
        if (isReady && token) {
            refreshTodos();
        }
    }, [refreshTodos, token, isReady]);

    // --- Event Handlers ---
    const handleAdd = async () => {
        if (!newTitle.trim()) return;
        await createTodo(getAuthHeaders, newTitle);
        setNewTitle("");
        await refreshTodos();
    };

    const handleToggleDone = async (todo: Todo) => {
        await updateTodo(getAuthHeaders, todo.id, todo.title, !todo.done);
        await refreshTodos();
    };

    const handleStartEdit = (todo: Todo) => {
        setEditingId(todo.id);
        setEditTitle(todo.title);
    };

    const handleSaveEdit = async (todo: Todo) => {
        if (!editTitle.trim()) return;
        await updateTodo(getAuthHeaders, todo.id, editTitle, todo.done);
        setEditingId(null);
        setEditTitle("");
        await refreshTodos();
    };

    const handleDelete = async (id: number) => {
        await deleteTodo(getAuthHeaders, id);
        await refreshTodos();
    };

    // WICHTIG: Loading State während isReady false ist
    if (!isReady) {
        return (
            <div className="max-w-xl mx-auto pt-10 text-center">
                <p className="text-gray-500">Lade...</p>
            </div>
        );
    }

    // --- Render ---
    return (
        <div className="max-w-xl mx-auto pt-10">
            {/* Header */}
            <div className="flex justify-between items-center mb-6">
                <h1 className="text-2xl font-bold">Todo List (Authenticated)</h1>
                <div className="flex gap-2 items-center">
                    <button
                        onClick={toggleDark}
                        className="px-3 py-1 rounded border dark:border-gray-600"
                    >
                        {isDark ? "☀️ Light" : "🌙 Dark"}
                    </button>
                    <button
                        onClick={logout}
                        className="bg-red-600 text-white px-3 py-1 rounded hover:bg-red-700 transition"
                    >
                        Logout
                    </button>
                </div>
            </div>

            {/* New Todo */}
            <div className="flex gap-2 mb-8">
                <input
                    value={newTitle}
                    onChange={(e) => setNewTitle(e.target.value)}
                    placeholder="Neue Aufgabe"
                    className="flex-1 border px-3 py-2 rounded 
                               bg-white dark:bg-gray-800 dark:text-white 
                               border-gray-300 dark:border-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <button
                    onClick={handleAdd}
                    className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
                    disabled={!newTitle.trim()}
                >
                    Hinzufügen
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
                            // Edit Mode
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
                                    Speichern
                                </button>
                                <button 
                                    onClick={() => setEditingId(null)}
                                    className="text-sm px-3 py-1 border border-gray-400 dark:border-gray-600 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
                                >
                                    Abbrechen
                                </button>
                            </div>
                        ) : (
                             // Display Mode
                            <div className="flex flex-1 items-center gap-3">
                                <input
                                    type="checkbox"
                                    checked={todo.done}
                                    onChange={() => handleToggleDone(todo)}
                                    className="h-5 w-5 rounded form-checkbox text-blue-600 dark:bg-gray-700"
                                />
                                <div className="flex-1">
                                    <span 
                                        className={`block ${todo.done ? 'line-through text-gray-500 dark:text-gray-400' : ''}`}
                                        onDoubleClick={() => handleStartEdit(todo)}
                                    >
                                        {todo.title}
                                    </span>
                                    <span className="text-xs text-gray-400 dark:text-gray-500">
                                        Score: {todo.score.toFixed(2)}
                                    </span>
                                </div>
                                <button
                                    onClick={() => handleStartEdit(todo)}
                                    className="text-sm text-blue-500 hover:text-blue-700 dark:hover:text-blue-400 p-1"
                                    title="Editieren"
                                >
                                    ✏️
                                </button>
                                <button
                                    onClick={() => handleDelete(todo.id)}
                                    className="text-sm text-red-500 hover:text-red-700 dark:hover:text-red-400 p-1 ml-1"
                                    title="Löschen"
                                >
                                    🗑️
                                </button>
                            </div>
                        )}
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default TodoList;
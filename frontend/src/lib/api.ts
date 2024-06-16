export async function request(url: string | URL, init: RequestInit) {
  return await fetch(`http://localhost:8000${url}`, init);
}

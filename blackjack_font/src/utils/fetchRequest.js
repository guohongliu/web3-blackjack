const fetchWithTimeout = (url, options = {}, timeout = 8000) => {
  // 览器原生提供的 API，主要用途是：控制 Fetch 请求（或其他支持 Abort 信号的异步任务）的中断 / 取消
  const controller = new AbortController();
  const timer = setTimeout(() => {
    controller.abort();
  }, timeout);

  return fetch(url, { ...options, signal: controller.signal }).finally(() =>
    clearTimeout(timer),
  );
};

const request = async (
  url,
  { method = "GET", body, header = {}, ...options } = {},
) => {
  try {
    const response = await fetchWithTimeout(url, {
      method,
      headers: {
        "Content-Type": "application/json",
        ...header,
      },
      body: JSON.stringify(body),
      ...options,
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      return Promise.reject({
        status: response.status,
        message: errorData.message || "请求失败",
      });
    }

    const contentType = response.headers.get("content-type") || "";
    if (contentType.includes("application/json")) {
      return response.json();
    }
    return response.text();
  } catch (error) {
    console.error("Error:", error);
    throw error;
  }
};

const api = {
  get: (endpoint, options) => request(endpoint, { method: "GET", ...options }),
  post: (endpoint, data, options) =>
    request(endpoint, { method: "POST", body: data, ...options }),
  put: (endpoint, data, options) =>
    request(endpoint, { method: "PUT", body: data, ...options }),
  delete: (endpoint, options) =>
    request(endpoint, { method: "DELETE", ...options }),
};

export default api;

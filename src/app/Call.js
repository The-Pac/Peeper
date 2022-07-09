import axios from "axios";

axios.defaults.baseURL = process.env.REACT_APP_CURRENT_BACKEND_URL
axios.defaults.headers = {
    "Access-Control-Allow-Origwith": "*"
}

export const Call = {
    withPost, withGet, withPut, withDelete, withPatch, withOptions, withHead
}

async function withPost(urlPath, datas) {
    await axios.post(urlPath, datas).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}

async function withGet(urlPath) {
    await axios.get(urlPath).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}

async function withPut(urlPath, datas) {
    await axios.put(urlPath, datas).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}

async function withDelete(urlPath) {
    await axios.delete(urlPath).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}

async function withPatch(urlPath, data) {
    await axios.patch(urlPath, data).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}

async function withOptions(urlPath) {
    await axios.options(urlPath).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}

async function withHead(urlPath) {
    await axios.head(urlPath).then(response => {
        return {
            data: response.data,
            headers: response.headers,
            request: response.request,
            status: response.status,
            statusText: response.statusText
        }
    })
}




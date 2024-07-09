<h1><a name="http">World http</a></h1>
<ul>
<li>Exports:
<ul>
<li>interface <a href="#dipankardas011_httpclient_outgoing_http_0_1_0"><code>dipankardas011:httpclient/outgoing-http@0.1.0</code></a></li>
</ul>
</li>
</ul>
<h2><a name="dipankardas011_httpclient_outgoing_http_0_1_0"></a>Export interface dipankardas011:httpclient/outgoing-http@0.1.0</h2>
<hr />
<h3>Types</h3>
<h4><a name="response"></a><code>record response</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="response.status_code"></a><code>status-code</code>: <code>u16</code></li>
<li><a name="response.headers"></a><code>headers</code>: <code>string</code></li>
<li><a name="response.body"></a><code>body</code>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a name="request_header"></a><code>record request-header</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="request_header.key"></a><code>key</code>: <code>string</code></li>
<li><a name="request_header.value"></a><code>value</code>: <code>string</code></li>
</ul>
<h4><a name="reserror"></a><code>record reserror</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="reserror.msg"></a><code>msg</code>: <code>string</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="get_request"></a><code>get-request: func</code></h4>
<h5>Params</h5>
<ul>
<li><a name="get_request.method"></a><code>method</code>: <code>string</code></li>
<li><a name="get_request.headers"></a><code>headers</code>: list&lt;<a href="#request_header"><a href="#request_header"><code>request-header</code></a></a>&gt;</li>
<li><a name="get_request.url"></a><code>url</code>: <code>string</code></li>
<li><a name="get_request.body"></a><code>body</code>: option&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_request.0"></a> result&lt;<a href="#response"><a href="#response"><code>response</code></a></a>, <a href="#reserror"><a href="#reserror"><code>reserror</code></a></a>&gt;</li>
</ul>

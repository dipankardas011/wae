<h1><a name="green">World green</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#dipankardas011_httpclient_outgoing_http_0_1_0"><code>dipankardas011:httpclient/outgoing-http@0.1.0</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#dipankardas011_watttime_watttime_0_1_0"><code>dipankardas011:watttime/watttime@0.1.0</code></a></li>
</ul>
</li>
</ul>
<h2><a name="dipankardas011_httpclient_outgoing_http_0_1_0"></a>Import interface dipankardas011:httpclient/outgoing-http@0.1.0</h2>
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
<h2><a name="dipankardas011_watttime_watttime_0_1_0"></a>Export interface dipankardas011:watttime/watttime@0.1.0</h2>
<hr />
<h3>Types</h3>
<h4><a name="point_data"></a><code>record point-data</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="point_data.point_time"></a><code>point-time</code>: <code>string</code></li>
<li><a name="point_data.value"></a><code>value</code>: <code>f32</code></li>
</ul>
<h4><a name="metadata_forecast"></a><code>record metadata-forecast</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="metadata_forecast.data_point_period_seconds"></a><code>data-point-period-seconds</code>: <code>s32</code></li>
<li><a name="metadata_forecast.region"></a><code>region</code>: <code>string</code></li>
<li><a name="metadata_forecast.warnings"></a><code>warnings</code>: list&lt;<code>string</code>&gt;</li>
<li><a name="metadata_forecast.signal_type"></a><code>signal-type</code>: <code>string</code></li>
<li><a name="metadata_forecast.model"></a><code>model</code>: <code>string</code></li>
<li><a name="metadata_forecast.units"></a><code>units</code>: <code>string</code></li>
<li><a name="metadata_forecast.generated_at_period_seconds"></a><code>generated-at-period-seconds</code>: <code>s32</code></li>
<li><a name="metadata_forecast.generated_at"></a><code>generated-at</code>: <code>string</code></li>
</ul>
<h4><a name="metadata_co2_moer_index"></a><code>record metadata-co2-moer-index</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="metadata_co2_moer_index.data_point_period_seconds"></a><code>data-point-period-seconds</code>: <code>s32</code></li>
<li><a name="metadata_co2_moer_index.region"></a><code>region</code>: <code>string</code></li>
<li><a name="metadata_co2_moer_index.warnings"></a><code>warnings</code>: list&lt;<code>string</code>&gt;</li>
<li><a name="metadata_co2_moer_index.signal_type"></a><code>signal-type</code>: <code>string</code></li>
<li><a name="metadata_co2_moer_index.model"></a><code>model</code>: <code>string</code></li>
<li><a name="metadata_co2_moer_index.units"></a><code>units</code>: <code>string</code></li>
</ul>
<h4><a name="forecast"></a><code>record forecast</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="forecast.data"></a><code>data</code>: list&lt;<a href="#point_data"><a href="#point_data"><code>point-data</code></a></a>&gt;</li>
<li><a name="forecast.meta"></a><code>meta</code>: <a href="#metadata_forecast"><a href="#metadata_forecast"><code>metadata-forecast</code></a></a></li>
</ul>
<h4><a name="co2_moer_index"></a><code>record co2-moer-index</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="co2_moer_index.data"></a><code>data</code>: <a href="#point_data"><a href="#point_data"><code>point-data</code></a></a></li>
<li><a name="co2_moer_index.meta"></a><code>meta</code>: <a href="#metadata_co2_moer_index"><a href="#metadata_co2_moer_index"><code>metadata-co2-moer-index</code></a></a></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="register"></a><code>register: func</code></h4>
<h5>Params</h5>
<ul>
<li><a name="register.username"></a><code>username</code>: <code>string</code></li>
<li><a name="register.password"></a><code>password</code>: <code>string</code></li>
<li><a name="register.email"></a><code>email</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="register.0"></a> <code>bool</code></li>
</ul>
<h4><a name="get_token"></a><code>get-token: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a name="get_token.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a name="get_region"></a><code>get-region: func</code></h4>
<h5>Params</h5>
<ul>
<li><a name="get_region.token"></a><code>token</code>: <code>string</code></li>
<li><a name="get_region.signal_type"></a><code>signal-type</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_region.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a name="get_forecast"></a><code>get-forecast: func</code></h4>
<h5>Params</h5>
<ul>
<li><a name="get_forecast.token"></a><code>token</code>: <code>string</code></li>
<li><a name="get_forecast.region"></a><code>region</code>: <code>string</code></li>
<li><a name="get_forecast.signal_type"></a><code>signal-type</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_forecast.0"></a> option&lt;<a href="#forecast"><a href="#forecast"><code>forecast</code></a></a>&gt;</li>
</ul>
<h4><a name="get_current_co2_moer_index"></a><code>get-current-CO2-MOER-index: func</code></h4>
<h5>Params</h5>
<ul>
<li><a name="get_current_co2_moer_index.token"></a><code>token</code>: <code>string</code></li>
<li><a name="get_current_co2_moer_index.region"></a><code>region</code>: <code>string</code></li>
<li><a name="get_current_co2_moer_index.signal_type"></a><code>signal-type</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_current_co2_moer_index.0"></a> option&lt;<a href="#co2_moer_index"><a href="#co2_moer_index"><code>co2-moer-index</code></a></a>&gt;</li>
</ul>

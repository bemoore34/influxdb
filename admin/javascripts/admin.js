!function(){var e;e=angular.module("adminApp",[]),e.controller("AdminIndexCtrl",["$scope","$location","$q",function(e,a,r){var t,n;return e.host=a.search().host||a.host(),e.port=a.search().port||("sandbox.influxdb.org"===e.host?9061:8086),e.database=a.search().database,e.username=a.search().username,e.password=a.search().password,e.authenticated=!1,e.data=[],e.readQuery=null,e.writeSeriesName=null,e.writeValues=null,e.successMessage="OK",e.alertMessage="Error",e.authMessage="",t=null,n=null,n=new InfluxDB(e.host,e.port,"root","root"),e.authenticate=function(){return t=new InfluxDB(e.host,e.port,e.username,e.password,e.database),r.when(t._readPoint("SELECT * FROM _foobar.bazquux_;")).then(function(r){return console.log(r),e.authenticated=!0,a.search({})},function(a){return e.authError(a.responseText)})},e.getDatabaseNames=function(){return r.when(n.getDatabaseNames()).then(function(a){return e.databases=JSON.parse(a)})},e.writeData=function(){var a;if(!e.writeSeriesName)return e.error("Time Series Name is required."),void 0;try{a=JSON.parse(e.writeValues)}catch(n){return e.alertMessage="Unable to parse JSON.",$("span#writeFailure").show().delay(1500).fadeOut(500),void 0}return r.when(t.writePoint(e.writeSeriesName,a)).then(function(){return e.success("200 OK")})},e.readData=function(){return e.data=[],r.when(t._readPoint(e.readQuery)).then(function(a){var r;return r=JSON.parse(a),r.forEach(function(a){return e.data.push({name:a.name,columns:a.columns,points:a.points})})})},e.authError=function(a){return e.authMessage=a,$("span#authFailure").show().delay(1500).fadeOut(500)},e.error=function(a){return e.alertMessage=a,$("span#writeFailure").show().delay(1500).fadeOut(500)},e.success=function(a){return e.successMessage=a,$("span#writeSuccess").show().delay(1500).fadeOut(500)},e.username&&e.password&&e.database?e.authenticate():void 0}])}.call(this);
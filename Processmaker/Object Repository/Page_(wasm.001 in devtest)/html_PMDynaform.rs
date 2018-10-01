<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_PMDynaform</name>
   <tag></tag>
   <elementGuidId>cd62b149-26da-4086-9105-b4c6be4f781e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>ext-gen60</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        PMDynaform
        
        
        
        
        
        
        
        
        

        (function(){(window.hostMIF = parent.document.getElementById(&quot;openCaseFrame&quot;).ownerCt)._windowContext={eval:function(s){return new Function(&quot;return (&quot;+s+&quot;)&quot;)();}};})()
        
        
        
        
        
        
        
    #katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}
    
        
        
	&lt;fieldset>
	    &lt;legend>Options&lt;/legend>
	    
	        &lt;div class='col-sm-12'>
	            &lt;div class='form-group'>
	            &lt;/div>
	        &lt;/div>
	&lt;/fieldset>



	&lt;div class=&quot;col-xs-&lt;%- colSpan %>&quot;>		  			 			 
	&lt;/div>



	&lt;div class=&quot;pmdynaform-field row show-grid&quot;>	      
	&lt;/div>



	&lt;div class=&quot;alert alert-danger&quot; role=&quot;alert&quot;>
		&lt;span class=&quot;glyphicon glyphicon-exclamation-sign&quot; aria-hidden=&quot;true&quot;>&lt;/span>
		&lt;span class=&quot;sr-only&quot;>Error:&lt;/span>
	  	&lt;% for (var item in message) {%>
			&lt;span>&lt;%-message[item]%> &lt;/span>
		&lt;%}%>
	&lt;/div>



	&lt;div class=&quot;alert alert-danger&quot; role=&quot;alert&quot;>
    	&lt;%-message%>
  	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%>  pmdynaform-field&lt;%}%>&quot;>
        &lt;% if(group === &quot;form&quot;) {%>
            &lt;label for=&quot;form[&lt;%-id%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%>  col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
            	&lt;span data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-tooltipLabel%>&quot; class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
	            &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
            &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
	  			class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
	  		&lt;%} else if (layout === &quot;responsive&quot;) {%>
	  			class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
	  		&lt;%}%>
		>
			&lt;input
				&lt;% if  ( group === &quot;grid&quot;) { %>
					id= &quot;form&lt;%-id%>&quot;
					name= &quot;form&lt;%-name%>&quot;
				&lt;%} else{%>
					&lt;% if (type === &quot;suggest&quot;){ %>
						id= &quot;form[&lt;%-id%>_label]&quot;
						name=&quot;form[&lt;%-name%>_label]&quot;
					&lt;%}else{%>
						id= &quot;form[&lt;%-id%>]&quot;
						name=&quot;form[&lt;%-name%>]&quot;
					&lt;%}%>
				&lt;%}%>
				type = &lt;%-type%>
				class=&quot;&lt;%-namespace%>-control-&lt;%-type%> form-control&quot;
				placeholder= &quot;&lt;%-placeholder%>&quot;
				&lt;%if(type === &quot;suggest&quot;){%>
					value = &quot;&lt;%-data['label']%>&quot;
				&lt;%}else{%>
					value = &quot;&lt;%-data['value']%>&quot;
				&lt;%}%>
				&lt;% if(disabled === true){%>disabled&lt;%}%>
				&lt;% if (autoComplete === &quot;on&quot; || autoComplete === &quot;off&quot;) { %>
					autocomplete = &quot;&lt;%-autoComplete%>&quot;
				&lt;%} else{%>
					autoComplete = &quot;off&quot;
				&lt;%}%>
			>
				&lt;% if (type === &quot;suggest&quot;){ %>
					&lt;input type=&quot;hidden&quot; value=&quot;&lt;%-data['value']%>&quot;
						&lt;% if  ( group === &quot;grid&quot;) { %>
							id = &quot;form&lt;%-id%>&quot;
							name =&quot;form&lt;%-name%>&quot;
						&lt;%} else{%>
							id = &quot;form[&lt;%-id%>]&quot;
							name =&quot;form[&lt;%-name%>]&quot;
						&lt;%}%>
					>
				&lt;%}%>
				&lt;% if (group === &quot;form&quot;) { %>
					&lt;%if (type === &quot;suggest&quot;){%>
						&lt;span class=&quot;spinner-icon&quot;>&lt;/span>
					&lt;%}%>
					&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
						&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
					&lt;%}%>
				&lt;% } %>
				&lt;% if (type !== &quot;suggest&quot;){ %>
					&lt;input type=&quot;hidden&quot;
						&lt;% if  ( group === &quot;grid&quot;) { %>
							id = &quot;form&lt;%-id%>&quot;
							name =&quot;form&lt;%-name%>&quot;
						&lt;%} else{%>
							id = &quot;form[&lt;%-id%>_label]&quot;
							name =&quot;form[&lt;%-name%>_label]&quot;
							value = &quot;&lt;%-data['label']%>&quot;
						&lt;%}%>
					>
				&lt;%}%>
		&lt;/div>
    &lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>

			&lt;label for=&quot;form[&lt;%-id%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
				&lt;span class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
				&lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
			&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
		  			class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		  		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		  			class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		  		&lt;%}%>>
			&lt;textarea
						&lt;% if  ( group === &quot;grid&quot;) { %>
							id = &quot;form&lt;%-id%>&quot;
							&lt;%if(name !== &quot;&quot;){%>
								name =&quot;form&lt;%-name%>&quot;
							&lt;%}%>
						&lt;%} else{%>
							&lt;%if(name !== &quot;&quot;){%>
								id = &quot;form[&lt;%-id%>]&quot;
								name =&quot;form[&lt;%-name%>]&quot;
							&lt;%}%>
						&lt;%}%>
						class=&quot;&lt;%-namespace%>-control-&lt;%-type%> form-control&quot;
                        &lt;% if ((new RegExp('^[0-9]+$')).test(rows)) { %>
                            rows=&quot;&lt;%- rows %>&quot;
                        &lt;% } else { %>
                            rows= &quot;2&quot;
                        &lt;% } %>
						placeholder= &quot;&lt;%- placeholder %>&quot;
				&lt;% if(disabled === true){%>disabled&lt;%}%>
			>&lt;%-value%>&lt;/textarea>
			&lt;% if (group === &quot;form&quot;) { %>
				&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
					&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
				&lt;%}%>
			&lt;% } %>
			&lt;input type=&quot;hidden&quot;
				&lt;% if  ( group === &quot;grid&quot;) { %>
					id = &quot;form&lt;%-id%>_label&quot;
					name =&quot;form&lt;%-name%>_label&quot;
					value=&quot;&lt;%-data['label']%>&quot;
				&lt;%} else{%>
					id = &quot;form[&lt;%-id%>_label]&quot;
					name =&quot;form[&lt;%-name%>_label]&quot;
 					value=&quot;&lt;%-data['label']%>&quot;
				&lt;%}%>
			>
		&lt;/div>
	&lt;/div>


 
	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
    	&lt;% if(group === &quot;form&quot;) {%>
	    	&lt;label for=&quot;form[&lt;%-id%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
	    		&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
	    		&lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
	    	&lt;/label>
	    &lt;%}%>
    	&lt;div &lt;%if (group === 'form'){ %>
		  			class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		  		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		  			class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		  		&lt;%}%>>
    		&lt;div class=&quot;pmdynaform-control-checkbox-list form-control&quot;>
    			&lt;div class=&quot;pmdynaform-checkbox-items&quot;>
	    			&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
						&lt;div class=&quot;checkbox&quot;>
				        	&lt;label>
				        		&lt;input
									&lt;% if  ( group === &quot;grid&quot;) { %>
										id = &quot;form&lt;%-id%>&quot;
										name =&quot;form&lt;%-name%>&quot;
									&lt;%} else {%>
										id = &quot;form[&lt;%-id%>][&lt;%-options[i].value%>]&quot;
										name =&quot;form[&lt;%-name%>][]&quot;
									&lt;%}%>
				        			class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;
					        		value=&quot;&lt;%-options[i].value%>&quot;
					        		type=&quot;checkbox&quot;

									&lt;% if(data.value &amp;&amp; data.value.length){
										for(var j=0; j&lt; data.value.length; j+=1) {
												if(data.value[j] === options[i].value){%>
												checked
								    &lt;%}}}%>

									&lt;% if(disabled === true){%>disabled&lt;%}%>>
					        		&lt;%if(dataType !== &quot;boolean&quot;){%>
										&lt;span>&lt;%-options[i].label%>&lt;/span>
					        		&lt;%}%>
				        	&lt;/label>
						&lt;/div>
				    &lt;% } %>
	    		&lt;/div>
				&lt;% if (group === &quot;form&quot;) { %>
					&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
						&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
					&lt;%}%>
				&lt;% } %>
    		&lt;/div>
			&lt;input type=&quot;hidden&quot;
				&lt;% if  ( group === &quot;grid&quot;) { %>
					id = &quot;form&lt;%-id%>_label&quot;
					name =&quot;form&lt;%-name%>_label&quot;
				&lt;%} else{%>
					id = &quot;form[&lt;%-id%>_label]&quot;
					name =&quot;form[&lt;%-name%>_label]&quot;
				&lt;%}%>
			>
		&lt;/div>
	&lt;/div>
 



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
		&lt;label for=&quot;form[&lt;%-id%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
			&lt;%if(required){%>
			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
			&lt;%}%>
		&lt;/label>
		&lt;%} else if((group !== &quot;form&quot;) &amp;&amp; (layout === &quot;form&quot;)){%>
		&lt;label for=&quot;&lt;%-name%>&quot; class=&quot;hidden-lg hidden-md hidden-sm visible-xs control-label pmdynaform-label col-xs-&lt;%-colSpanLabel%>&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
		&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
		class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%}%>>
		&lt;div class=&quot;pmdynaform-control-checkbox-list form-control&quot;>
			&lt;div class=&quot;pmdynaform-checkbox-items&quot;>
				&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
				&lt;div class=&quot;checkbox&quot;
				&lt;%if( i > 0){%>
				style = &quot;display:none&quot;
				&lt;%}%>
				>
				&lt;label>
					&lt;input
					&lt;% if  ( group === &quot;grid&quot;) { %>
					id = &quot;form&lt;%-id%>&quot;
					name =&quot;form&lt;%-name%>&quot;
					&lt;%} else {%>
					id = &quot;form[&lt;%-id%>]&quot;
					name =&quot;form[&lt;%-name%>][]&quot;
					&lt;%}%>
					class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;
					value=&quot;&lt;%-options[i].value%>&quot;
					type=&quot;checkbox&quot;
					&lt;% if(options[i].selected){%>checked&lt;%}%>
					&lt;% if(disabled === true){%>disabled&lt;%}%>>
				&lt;/label>
			&lt;/div>
			&lt;% } %>
		&lt;/div>
		&lt;% if (group === &quot;form&quot;) { %>
		&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
		&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
		&lt;%}%>
		&lt;% } %>
	&lt;/div>
	&lt;input type=&quot;hidden&quot;
	&lt;% if  ( group === &quot;grid&quot;) { %>
	id = &quot;form&lt;%-id%>_label&quot;
	name =&quot;form&lt;%-name%>_label&quot;
	&lt;%} else{%>
	id = &quot;form[&lt;%-id%>_label]&quot;
	name =&quot;form[&lt;%-name%>_label]&quot;
	&lt;%}%>
	>
	&lt;/div>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
		&lt;label for=&quot;form[&lt;%-name%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
			&lt;%if(required){%>
			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
			&lt;%}%>
		&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
		class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		&lt;%} else if (layout === &quot;form&quot;){ %>
		class=&quot;col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%}%>>
		&lt;div class=&quot;pmdynaform-control-radio-list&quot;>
			&lt;div class=&quot;pmdynaform-radio-items&quot;>
				&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
				&lt;div class=&quot;radio&quot;>
					&lt;label>
						&lt;input
						&lt;% if  ( group === &quot;grid&quot;) { %>
						id = &quot;form&lt;%-id%>&quot;
						name =&quot;form&lt;%-name%>&quot;
						&lt;%} else{%>
						id = &quot;form[&lt;%-id%>]&quot;
						name =&quot;form[&lt;%-name%>]&quot;
						&lt;%}%>
						class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;
						value=&quot;&lt;%-options[i].value%>&quot;
						type=&quot;&lt;%-type%>&quot;
						&lt;% if(value == options[i].value){%>checked&lt;%}%>
						&lt;% if(disabled === true){%>disabled&lt;%}%>
						>
						&lt;span>&lt;%-options[i].label%>&lt;/span>
					&lt;/label>
				&lt;/div>
				&lt;% } %>
			&lt;/div>
			&lt;input type=&quot;hidden&quot; value=&quot;&lt;%-data['label']%>&quot;
			&lt;% if  ( group === &quot;grid&quot;) { %>
			id = &quot;form&lt;%-id%>_label&quot;
			name =&quot;form&lt;%-name%>_label&quot;
			&lt;%} else{%>
			id = &quot;form[&lt;%-id%>_label]&quot;
			name =&quot;form[&lt;%-name%>_label]&quot;
			&lt;%}%>
			>
			&lt;% if (group === &quot;form&quot;) { %>
			&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
			&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
			&lt;%}%>
			&lt;% } %>
		&lt;/div>
	&lt;/div>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
		&lt;label for=&quot;form[&lt;%-id%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
			&lt;%if(required &amp;&amp; enableValidate){%>
			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
			&lt;%}%>
		&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
		class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		&lt;%} else if (layout === &quot;form&quot;){ %>
		class=&quot;pmdynaform-dropdown-control col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		class=&quot;pmdynaform-dropdown-control col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%}%>>
		&lt;select
		&lt;% if  ( group === &quot;grid&quot;) { %>
		id = &quot;form&lt;%-id%>&quot;
		name =&quot;form&lt;%-name%>&quot;
		&lt;%} else{%>
		id = &quot;form[&lt;%-id%>]&quot;
		name =&quot;form[&lt;%-name%>]&quot;
		&lt;%}%>
		class=&quot;&lt;%-namespace%>-control-&lt;%-type%> form-control&quot;
		&lt;% if(disabled === true){%>disabled&lt;%}%>>
		&lt;%if(therePlaceholder){%>
		&lt;option disabled id=&quot;placeholder-option&quot; value=&lt;%-placeholderOption[&quot;value&quot;]%> >&lt;%-placeholderOption[&quot;label&quot;]%>&lt;/option>
		&lt;%}%>
		&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
		&lt;option value=&quot;&lt;%-options[i].value%>&quot;
		&lt;%if( options[i].value == value ){%>
		selected&lt;%}%>
		>&lt;%-options[i].label%>&lt;/option>
		&lt;% } %>
		&lt;/select>
		&lt;% if (group === &quot;form&quot;) { %>
		&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
		&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
		&lt;%}%>
		&lt;% } %>
		&lt;input type=&quot;hidden&quot; value=&quot;&lt;%-data['label']%>&quot;
		&lt;% if  ( group === &quot;grid&quot;) { %>
		id = &quot;form&lt;%-id%>_label&quot;
		name =&quot;form&lt;%-name%>_label&quot;
		&lt;%} else{%>
		id = &quot;form[&lt;%-id%>_label]&quot;
		name =&quot;form[&lt;%-name%>_label]&quot;
		&lt;%}%>
		>
	&lt;/div>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> text-center&quot;>
		&lt;button id=&quot;form[&lt;%-id%>]&quot; name=&quot;form[&lt;%-name%>]&quot; type=&lt;%-type%> class=&quot;btn btn-primary&quot;
			&lt;% if(disabled === true){%>disabled&lt;%}%>>
			&lt;span> &lt;%-label%> &lt;/span>
		&lt;/button>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> text-center&quot;>
		&lt;button id=&quot;form[&lt;%-id%>]&quot; name=&quot;form[&lt;%-name%>]&quot; type=&lt;%-type%> class=&quot;btn btn-default&quot;
			&lt;% if(disabled === true){%>disabled&lt;%}%>>
			&lt;span> &lt;%-label%> &lt;/span>
		&lt;/button>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
		&lt;label for=&quot;form[&lt;%-name%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
			&lt;%if(required){%>
			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
			&lt;%}%>
		&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if(group === 'form'){ %> class=&quot;pmdynaform-file-control col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot; &lt;%}%> >
		&lt;div class=&quot;pmdynaform-file-container col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;>
			&lt;div class=&quot;pmdynaform-file-btn-container&quot;>
				&lt;button type=&quot;button&quot; class=&quot;&lt;%-namespace%>-control-&lt;%-type%> form-control&quot;
				&lt;% if(disabled === true){%>disabled&lt;%}%>
				&lt;% if(mode === &quot;view&quot;){%>style=&quot;display:none;&quot;&lt;%}%>
				>
				&lt;%-labelButton%>
				&lt;/button>
			&lt;/div>
			&lt;input
			&lt;% if  ( group === &quot;grid&quot;) { %>
			id = &quot;form&lt;%-id%>&quot;
			name =&quot;form&lt;%-name%>&quot;
			&lt;%} else{%>
			id = &quot;form[&lt;%-name%>]&quot;
			name =&quot;form[&lt;%-name%>]&quot;
			&lt;%}%>
			type=&quot;file&quot; style=&quot;visibility:hidden;&quot;
			&lt;% if(disabled === true){%>disabled&lt;%}%>
			&lt;% if(multiple === true){%>multiple&lt;%}%>
			>
			&lt;%if (group === &quot;form&quot;){%>
			&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
			&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
			&lt;%}%>
			&lt;%}%>
			&lt;input type=&quot;hidden&quot;
			&lt;% if  ( group === &quot;grid&quot;) { %>
			id = &quot;form&lt;%-id%>_label&quot;
			name =&quot;form&lt;%-name%>_label&quot;
			&lt;%} else {%>
			id = &quot;form[&lt;%-id%>_label]&quot;
			name =&quot;form[&lt;%-name%>_label]&quot;
			&lt;%}%>
			>
			&lt;%if(data['value'].length){%>
			&lt;div class= &quot;col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> link-container pmdynaform-field-control&quot;>
				&lt;% for(var i = 0; i &lt; data['label'].length; i+=1) { %>
				&lt;a style=&quot;float:left&quot;
				   href = &quot;&lt;%-data['value'][i]%>&quot; class=&quot;btn btn-link &lt;%-namespace%>-control-&lt;%-type%>&quot;>
					&lt;span> &lt;%-data['label'][i]%>&lt;/span>
				&lt;/a>
				&lt;% } %>
			&lt;/div>
			&lt;%}%>
		&lt;/div>
	&lt;/div>
	&lt;/div>



	&lt;ul class=&quot;list-group col-lg-11 col-sm-11 col-md-11 col-xs-10 pmdynaform-suggest-list&quot;>
	&lt;/ul>



	&lt;span data-value=&quot;&lt;%-value %>&quot; class=&quot;list-group-item&quot;>&lt;%- label %>&lt;/span>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%>  &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
        &lt;% if(group === &quot;form&quot;) {%>
            &lt;label for=&quot;form[&lt;%-id%>]&quot; class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
            	&lt;span class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
	            &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
            &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
		  			class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		  		&lt;%} else if (layout === &quot;form&quot;){ %>
		  			class=&quot;col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12&quot;
		  		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		  			class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		  		&lt;%}%>>
			&lt;a
				&lt;% if  ( group === &quot;grid&quot;) { %>
					id = &quot;form&lt;%-id%>&quot;
				&lt;%} else{%>
					id = &quot;form[&lt;%-id%>]&quot;
				&lt;%}%>
				class = &quot;pmdynaform-link form-control&quot;
				href=&quot;&lt;%-href%>&quot; target=&quot;&lt;%-target%>&quot; class=&quot;btn btn-link &lt;%-namespace%>-control-&lt;%-type%>&quot;
				&lt;% if(disabled === true){%>disabled&lt;%}%>>
				&lt;span> &lt;%-text%> &lt;/span>
			&lt;/a>
			&lt;% if (group === &quot;form&quot;) { %>
				&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
					&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
				&lt;%}%>
			&lt;% } %>
		&lt;/div>
    &lt;/div>
 


	&lt;div class=&quot;form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> tpl-empty&quot;>
	&lt;/div>


 
	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> pmdynaform-field&quot;>
		&lt;h4 id=&quot;form[&lt;%-id%>]&quot;>
			&lt;p class=&quot;&lt;%-className[type]%>&quot;>
				&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
			&lt;/p>
		&lt;/h4>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> pmdynaform-field&quot;>
		&lt;h5 id=&quot;form[&lt;%-id%>]&quot;>
			&lt;p class=&quot;&lt;%-className[type]%>&quot;>
				&lt;span class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
			&lt;/p>
		&lt;/h5>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-originalType%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
		&lt;label class=&quot;col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%-label %>&lt;/span>
			&lt;%if(required){%>
			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
			&lt;%}%>
		&lt;/label>
		&lt;%} else if((group !== &quot;form&quot;) &amp;&amp; (layout === &quot;form&quot;)){%>
		&lt;label class=&quot;hidden-lg hidden-md hidden-sm visible-xs control-label pmdynaform-label col-xs-&lt;%-colSpanLabel%>&quot;>
			&lt;span class=&quot;textlabel&quot;>&lt;%-label %>&lt;/span>
		&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
		class=&quot;col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		&lt;%} else if (layout === &quot;form&quot;){ %>
		class=&quot;col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		&lt;%}%>>
		&lt;div class=&quot;pmdynaform-label-options form-control&quot;>
			&lt;%if(originalType==&quot;suggest&quot;){%>
			&lt;%}%>
			&lt;%for(var k=0; k&lt;fullOptions.length; k+=1){%>
			&lt;span title=&quot;&lt;%-fullOptions[k]%>&quot;
			&lt;%if(originalType !== '') {%>
			class = &quot;label-&lt;%-originalType%>&quot;
			&lt;%} else {%>
			class = &quot;label-control&quot;
			&lt;%}%>
			>&lt;%-fullOptions[k]%>&lt;/span>
			&lt;%}%>
            &lt;% if (originalType === &quot;checkgroup&quot;) {
            _.each(data['value'], function (item, index) { %>
                &lt;input type=&quot;hidden&quot; value=&quot;&lt;%-item%>&quot; name=&quot;form[&lt;%-name%>][]&quot; id=&quot;form[&lt;%-id%>][&lt;%-index%>]&quot;/>
            &lt;% });
            } else {
            %>
                &lt;input class=&quot;value-hidden&quot; type=&quot;hidden&quot; value=&quot;&lt;%-data['value']%>&quot;
                &lt;% if  ( group === &quot;grid&quot;) { %>
                id = &quot;form&lt;%-id%>&quot;
                name =&quot;form&lt;%-name%>&quot;
                &lt;%} else{%>
                id = &quot;form[&lt;%-id%>]&quot;
                name =&quot;form[&lt;%-name%>]&quot;
                &lt;%}%>
                >
            &lt;% } %>
			&lt;input class=&quot;label-hidden&quot; type=&quot;hidden&quot; value=&quot;&lt;%-data['label']%>&quot;
			&lt;% if  ( group === &quot;grid&quot;) { %>
			id = &quot;form&lt;%-id%>_label&quot;
			name =&quot;form&lt;%-name%>_label&quot;
			&lt;%} else{%>
			id = &quot;form[&lt;%-id%>_label]&quot;
			name =&quot;form[&lt;%-name%>_label]&quot;
			&lt;%}%>
			>
		&lt;/div>
		&lt;% if (group === &quot;form&quot;) { %>
		&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
		&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%=hint%>&quot;>&lt;/span>
		&lt;%}%>
		&lt;% } %>
	&lt;/div>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> pmdynaform-field&quot;>
		&lt;input
		&lt;% if  ( group === &quot;grid&quot;) { %>
		id = &quot;form&lt;%-id%>&quot;
		name =&quot;form&lt;%-name%>&quot;
		&lt;%} else{%>
		id = &quot;form[&lt;%-id%>]&quot;
		name =&quot;form[&lt;%-name%>]&quot;
		&lt;%}%>
		class=&quot;&lt;%-namespace%>-control-&lt;%-type%> form-control&quot;
		type=&quot;hidden&quot;
		value= &quot;&lt;%-value%>&quot;>
		&lt;input type=&quot;hidden&quot; value=&quot;&lt;%-data['value']%>&quot;
		&lt;% if  ( group === &quot;grid&quot;) { %>
		id = &quot;form&lt;%-id%>_label&quot;
		name =&quot;form&lt;%-name%>_label&quot;
		&lt;%} else{%>
		id = &quot;form[&lt;%-id%>_label]&quot;
		name =&quot;form[&lt;%-name%>_label]&quot;
		&lt;%}%>
		>
	&lt;/div>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
        &lt;label class=&quot;col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
        	&lt;span class=&quot;textlabel&quot;>&lt;%-label %>&lt;/span>
        &lt;/label>
        &lt;div class=&quot;&lt;%-namespace%>-control-&lt;%-type%> col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;>
        	&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%> control&quot;>
        		&lt;img id=&quot;PMD-&lt;%-id%>&quot; class=&quot;img-responsive &lt;%-shape%>&quot; src=&quot;&lt;%-src%>&quot; alt=&quot;&lt;%-alternateText%>&quot; title=&quot;&lt;%-alt%>&quot; scale=&quot;0&quot;>
        		&lt;span class=&quot;pmdynaform-image-comment text-primary&quot;>&lt;%-comment%>&lt;/span>
        	&lt;/p>
			&lt;% if (group === &quot;form&quot;) { %>
			&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
			&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
			&lt;%}%>
			&lt;% } %>
		&lt;/div>
	&lt;/div>



	&lt;audio class=&quot;embed-responsive-item&quot; src=&quot;&lt;%-path%>&quot; width=&quot;200&quot; height=&quot;200&quot; autobuffer autoplay controls>
		&lt;p>Your browser does not support the audio element &lt;/p>
	&lt;/audio>



	&lt;video class=&quot;embed-responsive-item&quot; src=&quot;&lt;%-path%>&quot;  width=&quot;200&quot; height=&quot;200&quot; autoplay autobuffer controls>
		&lt;p>Your browser does not support the video element &lt;/p>
	&lt;/video>



	&lt;div name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%>&quot;>
	&lt;/div>



	&lt;div class=&quot;pmdynaform-form-message-loading&quot;>
		&lt;h3>&lt;%-title%>&lt;/h3>
		&lt;div class=&quot;alert alert-warning&quot; role=&quot;alert&quot;>
			&lt;span class=&quot;glyphicon glyphicon-refresh glyphicon-refresh-animate&quot;>&lt;/span>
			&lt;strong>&lt;%-msg%>&lt;/strong>
		&lt;/div>

	&lt;/div>
	&lt;div id=&quot;shadow-form&quot;>&lt;/div>



    &lt;div id='&lt;%-id%>' class='&lt;%-namespace%>-field-&lt;%-type%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> pmdynaform-field &lt;%}%> '>
		&lt;% if(group === 'form') {%>
            &lt;label for='form[&lt;%-name%>]' class='col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label'>
				&lt;span data-toggle='tooltip' data-placement='bottom' class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
	            &lt;%if(required){%>
		  			&lt;span class='pmdynaform-field-required'>*&lt;/span>
		  		&lt;%}%>
            &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %>
	  			class='col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control'
	  		&lt;%} else if (layout === 'form'){ %>
	  			class='col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12'
	  		&lt;%} else if (layout === 'responsive') {%>
	  			class='col-xs-12 col-sm-12 col-md-12 col-lg-12'
	  		&lt;%}%>>
			    &lt;div class='datetime-container'>
					&lt;div id ='datetime-container-control' class='input-group date' &lt;%if (group === 'form'){ %>
				  			class='form-control'
				  		&lt;%} else if (layout === 'form'){ %>
				  			class='col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12'
				  		&lt;%} else if (layout === 'responsive') {%>
				  			class='col-xs-12 col-sm-12 col-md-12 col-lg-12'
				  		&lt;%}%>>
				        &lt;input type='text' class='pmdynaform-control-&lt;%-type%> form-control' readonly=&quot;readonly&quot;
							&lt;% if  ( group === 'grid') { %>
								id = 'form&lt;%-id%>'
								name ='form&lt;%-name%>'
							&lt;%} else{%>
								id = 'form[&lt;%-id%>]'
								name ='form[&lt;%-name%>_label]'
							&lt;%}%>
				    			class='&lt;%-namespace%>-control-&lt;%-
				    			type%> form-control'
				    			type='text'
				    			placeholder= '&lt;%- placeholder %>'
								value= '&lt;%-value%>'
				    			&lt;% if(disabled === true){%>disabled&lt;%}%>
				    			>
				    	&lt;/input>
						&lt;input type='hidden'
							&lt;% if  ( group === 'grid') { %>
								id = 'form&lt;%-id%>'
								name ='form&lt;%-name%>'
							&lt;%} else{%>
								id = 'form[&lt;%-id%>_label]'
								name ='form[&lt;%-name%>]'
								value = '&lt;%-data['label']%>'
							&lt;%}%>
						>
				        &lt;span class='input-group-addon'>&lt;span class='&lt;%if (format === 'LT'){%>glyphicon glyphicon-time&lt;%}else{%>glyphicon glyphicon-calendar&lt;%}%>'>
					        &lt;/span>
						&lt;/span>
					&lt;/div>
				&lt;/div>
	&lt;% if (group === 'form') { %>
	&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
	&lt;span class='glyphicon glyphicon-info-sign' data-toggle='tooltip' data-placement='bottom' title='&lt;%-hint%>'>&lt;/span>
	&lt;%}%>
	&lt;% } %>
	&lt;/div>
	&lt;/div>



	&lt;%if(therePlaceholder){%>
	&lt;option disabled id=&quot;placeholder-option&quot; value=&lt;%-placeholderOption[&quot;value&quot;]%> >&lt;%-placeholderOption[&quot;label&quot;]%>&lt;/option>
	&lt;%}%>
	&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
	&lt;option value= &quot;&lt;%-options[i].value%>&quot;>
		&lt;%-options[i].label%>
	&lt;/option>
	&lt;% } %>




	&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
	&lt;div class=&quot;radio&quot;>
		&lt;label>
			&lt;input
					id= &quot;form[&lt;%-id%>][&lt;%-i%>]&quot;
					name= &quot;form[&lt;%-name%>]&quot;
					class= &quot;pmdynaform-control-&lt;%-type%>&quot;
					value= &quot;&lt;%-options[i].value%>&quot;
					type= &quot;&lt;%-type%>&quot;
			&lt;% if(disabled) {%> disabled &lt;%}%>
			>
			&lt;span>&lt;%-options[i].label%>&lt;/span>
		&lt;/label>
	&lt;/div>
	&lt;% } %>



	&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
	&lt;div class=&quot;checkbox&quot;>
		&lt;label>
			&lt;input
					id= &quot;form[&lt;%-id%>][&lt;%-i%>]&quot;
					name= &quot;form[&lt;%-name%>][]&quot;
					class= &quot;pmdynaform-control-&lt;%-type%>&quot;
					value= &quot;&lt;%-options[i].value%>&quot;
					type= &quot;&lt;%-type%>&quot;
			&lt;% if(disabled) {%> disabled &lt;%}%>
			>
			&lt;span>&lt;%-options[i].label%>&lt;/span>
		&lt;/label>
	&lt;/div>
	&lt;% } %>



	&lt;% for(var i=0; i&lt;options.length; i+=1) { %>
		&lt;li class=&quot;list-group-item&quot;>
			&lt;a href=&quot;#&quot; data-value=&quot;&lt;%-options[i].value%>&quot; data-label=&quot;&lt;%if(options[i].text){%>&lt;%-options[i].text%>&lt;%}else{%>&lt;%-options[i].label%>&lt;%}%>&quot; selected=&quot;false&quot;>
				&lt;%if(options[i].text){%>
				&lt;%-options[i].text%>
				&lt;%}else{%>
				&lt;%-options[i].label%>
				&lt;%}%>
			&lt;/a>
		&lt;/li>
	&lt;% } %>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> &lt;%-layout%>&quot;>
		&lt;div class=&quot;&lt;%-namespace%>-&lt;%-type%>-new&quot;>
			&lt;p class=&quot;pmdynaform-grid-title&quot;>&lt;span>&lt;%-title%>&lt;/span>
				&lt;%if (required) {%>
				&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
				&lt;%}%>
				&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null) {%>
					&lt;span style=&quot;float:right; margin-left:30px;&quot; class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
				&lt;%}%>
			&lt;/p>
			&lt;% if  ( mode !== &quot;view&quot; &amp;&amp; mode !== &quot;disabled&quot; &amp;&amp; addRow) { %>
				&lt;button type=&quot;button&quot; class=&quot;&lt;%-namespace%>-&lt;%-type%>-newitem&quot;>
					&lt;span class=&quot;pmdynaform-grid-plus glyphicon glyphicon-plus glyphicon btn btn-default btn-sm&quot;>&lt;/span>
					&lt;span class=&quot;pmdynaform-grid-text-plus&quot;>&lt;%-addRowText%>&lt;/span>
				&lt;/button>

			&lt;%}%>
		&lt;/div>
		&lt;div class=&quot;pmdynaform-grid-fields&quot;>
			&lt;div class=&quot;pmdynaform-grid&quot;>
				&lt;div class=&quot;row pmdynaform-grid-thead&quot;>&lt;/div>
				&lt;div id=&quot;&lt;%-id%>-body&quot; &lt;% if(pager) {%> class=&quot;carousel slide&quot; data-ride=&quot;carousel&quot; &lt;%}%> >
					&lt;div class=&quot;&lt;% if(pager) {%>carousel-inner &lt;%}%> pmdynaform-grid-tbody pmdynaform-form&quot;>&lt;/div>
					&lt;%if (functions) {%>
						&lt;%if (layout === &quot;static&quot;){%>
							&lt;div class=&quot;containerStaticGrid&quot; style=&quot;overflow:hidden&quot;>
						&lt;%}%>
						&lt;div class=&quot;pmdynaform-grid-functions&quot;>&lt;/div>
						&lt;%if (layout === &quot;static&quot;){%>
							&lt;/div>
						&lt;%}%>
					&lt;%}%>
					&lt;%if (pager) {%>
						&lt;div class=&quot;pmdynaform-grid-pagination&quot;>
						&lt;/div>
					&lt;%}%>
				&lt;/div>
			&lt;/div>
		&lt;/div>
	&lt;/div>



 	&lt;ul class=&quot;pagination&quot;>
		&lt;li class=&quot;toFirst&quot;>&lt;a data-target=&quot;#&lt;%-id%>&quot; data-slide-to=&quot;0&quot; href=&quot;#&quot;>&amp;laquo;&lt;/a>&lt;/li>
		&lt;% 
		var pointer;
		var display = &quot;&quot;; 
		var sectionSize = 5;
		var name;
		for (var i=0; i&lt;paginationItems; i+=1) {
		    pointer = i+1;
    		if(pointer > sectionSize &amp;&amp; !paginationRotate) {
                display = &quot;none&quot;;
    		}
		%>	
			&lt;% 
				name = &quot;sec_&quot; + i
			%>
			&lt;li 
				&lt;%if(i === 0){%>
					class = &quot;sec_&lt;%=i+1%>&quot;
				&lt;%}else{%>
					class = &quot;sec_&lt;%=i+1%>&quot;
				&lt;%}%>
			>
			    &lt;a data-target=&quot;#&lt;%-id%>&quot; data-slide-to=&quot;&lt;%-i%>&quot; href=&quot;&quot;>&lt;%-pointer%>&lt;/a>
			&lt;/li>
		&lt;%}%>
		&lt;li class=&quot;toLast&quot;>&lt;a data-target=&quot;#&lt;%-id%>&quot; data-slide-to=&quot;&lt;%-paginationItems-1%>&quot; href=&quot;&quot;>&amp;raquo;&lt;/a>&lt;/li>
	&lt;/ul>



	&lt;%for(var j=0; j&lt;totalrow.length; j+=1){%>
		&lt;div class=&quot;col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%>&quot;>
			&lt;span> &lt;%=totalrow[j]? totalrow[j] : 0 %> &lt;/span>
		&lt;/div>
	&lt;%}%>



	&lt;div class=&quot;grid-empty&quot;>
		&lt;span class=&quot;grid-empty-message&quot;> &lt;%=message%> &lt;/span>
	&lt;/div>


	&lt;div id=&quot;&lt;%=id%>&quot; name=&quot;&lt;%=name%>&quot; class=&quot;pmdynaform-field-geomap &lt;%=namespace%>-&lt;%=mode%>-&lt;%=type%> form-group col-sm-&lt;%=colSpan%> col-md-&lt;%=colSpan%> col-lg-&lt;%=colSpan%> pmdynaform-field&quot;>
		&lt;label class=&quot;col-sm-&lt;%=colSpanLabel%> col-md-&lt;%=colSpanLabel%> col-lg-&lt;%=colSpanLabel%> control-label pmdynaform-label&quot;>
        	&lt;span data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; class=&quot;textlabel&quot;>&lt;%-label%>&lt;/span>
            &lt;%if(required){%>
	  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
	  		&lt;%}%>
        &lt;/label>
        &lt;div class=&quot;col-sm-&lt;%=colSpanControl%> col-md-&lt;%=colSpanControl%> col-lg-&lt;%=colSpanControl%> pmdynaform-field-control&quot;>
            &lt;% if(disabled === true) {%>
                &lt;div class=&quot;pmdynaform-map-layer-disabled&quot;>&lt;/div>
            &lt;%}%>
        	&lt;div id=&quot;PMD-&lt;%-id%>&quot; class=&quot;pmdynaform-map-canvas&quot;>
        	&lt;/div>
        	&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
                &lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%=hint%>&quot;>&lt;/span>
			&lt;%}%>
        &lt;/div>
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> pmdynaform-field&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%-label%>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>





	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> alert alert-warning&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%- label %>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%=namespace%>-field-&lt;%=type%> form-group col-sm-&lt;%=colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%=colSpan%> alert alert-warning&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%-label%>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> alert alert-warning&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%-label%>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> alert alert-warning&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%- label %>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> alert alert-warning&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%- label %>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> alert alert-warning&quot; role=&quot;alert&quot;>
		&lt;span>
			&lt;p class=&quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;>
				&lt;span>&lt;%- label %>&lt;/span>
			&lt;/p>
		&lt;/span>				
	&lt;/div>


	&lt;div id=&lt;%-id%> class=&quot;panel panel-&lt;%-typePanel%> &lt;%-namespace%>-&lt;%-type%>  &lt;%-namespace%>-field-&lt;%-type%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> pmdynaform-field&quot; &lt;% if (border) { %> style=&quot;border-width:&lt;%-border%>&quot; &lt;%} %>>
		&lt;div class=&quot;panel-heading&quot; style=&quot;&lt;%if(!showHeader){%>display:none;&lt;%}%>&quot;>
			&lt;h3 class=&quot;panel-title&quot;>
				&lt;div class=&quot;header-content&quot;>
					&lt;span>&lt;%-title%>&lt;/span>
				&lt;/div>
			&lt;/h3>
		&lt;/div>
	&lt;div class=&quot;panel-body&quot;>
		&lt;div class = &quot;content-body&quot;>

		&lt;/div>
	&lt;/div>
		&lt;div class=&quot;panel-footer&quot; style=&quot;&lt;%if(!showFooter){%>display:none&lt;%}%>&quot;>
			&lt;div class=&quot;footer-content&quot;>

			&lt;/div>
		&lt;/div>
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
			&lt;label class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
	        	&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
	            &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
	        &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %> 
		  			class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		  		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		  			class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		  		&lt;%}%>>
		  		&lt;scanner id=&quot;PMD-&lt;%-id%>&quot;>
				&lt;div class=&quot;pmdynaform-label-options&quot;>
							&lt;span>&lt;%-value%>&lt;/span>							
				&lt;/div>
				&lt;/scanner>
		  		&lt;button type=&quot;button&quot; class=&quot;btn btn-default btn-md&quot;>
				  	&lt;span class=&quot;glyphicon glyphicon-qrcode&quot;>&lt;/span> &lt;%-labelButton%>
				&lt;/button>
				
			&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
				&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
			&lt;%}%>
		&lt;/div>
	&lt;/div>




	&lt;div class=&quot;pmdynaform-file-containerimage&quot;>
		&lt;buttonImage>
		&lt;span class= &quot;pmdynaform-file-preview&quot;>
			&lt;div class = &quot;pmdynaform-file-resizeimage-plus&quot;>+
			&lt;/div>
		&lt;/span>
		&lt;/buttonImage>
	&lt;/div>



	&lt;span>
		&lt;%-label%>
	&lt;/span>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
	    &lt;% if(group === &quot;form&quot;) {%>
		    &lt;label class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
		    	&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
		        &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
		    &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if(group === 'form'){ %>
			class=&quot;pmdynaform-file-control col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		&lt;%}%> >
			&lt;div class=&quot;pmdynaform-file-container pmdynaform-&lt;%-type%>-container&quot;>
				&lt;button type=&quot;button&quot; class=&quot;&lt;%-namespace%>-control-&lt;%-type%> form-control&quot; style=&quot;display:none&quot;>&lt;%-labelButton%>&lt;/button>
				&lt;input type=&quot;file&quot; style=&quot;display:none;&quot;
					&lt;% if(disabled === true){%>disabled&lt;%}%>
					&lt;% if(multiple === true){%>multiple&lt;%}%>
				>
				&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
					&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
				&lt;%}%>
				&lt;% if(dnd) {%>
					&lt;p style=&quot;float:left;&quot; class=&quot;help-block&quot;>&lt;%-dndMessage%>&lt;/p>
				&lt;%}%>
					&lt;div class=&quot;pmdynaform-file-droparea-ext&quot;>&lt;/div>
			&lt;/div>
		&lt;/div>
	&lt;/div>




	&lt;div class=&quot;pmdynaform-file-containervideo&quot;>
		&lt;div class =&quot;pmdynaform-file-nameaudio&quot;>
			&lt;span>&lt;%-name%>&lt;/span>
		&lt;/div>
	&lt;span class=&quot;pmdynaform-file-preview&quot; style=&quot;float: left;&quot;>
    	&lt;div class=&quot;pmdynaform-file-resizevideo&quot;>
		&lt;/div>
	&lt;/span>
&lt;/div>





	&lt;div  class=&quot;pmdynaform-file-containeraudio&quot;>
		&lt;div class =&quot;pmdynaform-file-nameaudio&quot;>
			&lt;span>&lt;%-name%>&lt;/span>
		&lt;/div>
    	&lt;div class=&quot;pmdynaform-file-resizevideo&quot;>
		&lt;/div>
&lt;/div>




	
	&lt;div class=&quot;pmdynaform-file-containerimage&quot;>
		&lt;buttonImage>	
		&lt;span class= &quot;pmdynaform-file-preview&quot;>
			&lt;div class = &quot;pmdynaform-file-resizeaudio-plus&quot;>
			&lt;span class=&quot;glyphicon glyphicon-play-circle&quot; style=&quot;margin:5px&quot;>&lt;/span>
			&lt;/div>
		&lt;/span>
		&lt;/buttonImage>	
	&lt;/div>	


	
	&lt;div class=&quot;pmdynaform-file-containerimage&quot;>
		&lt;buttonImage>	
		&lt;span class= &quot;pmdynaform-file-preview&quot;>
			&lt;div class = &quot;pmdynaform-file-resizeaudio-plus&quot;>
			&lt;span class=&quot;glyphicon glyphicon-film&quot; style=&quot;margin:5px&quot;>&lt;/span>
			&lt;/div>
		&lt;/span>
		&lt;/buttonImage>	
	&lt;/div>	


	
	&lt;div class=&quot;pmdynaform-file-containerimage&quot;>
		&lt;buttonImage>	
		&lt;span class= &quot;pmdynaform-file-preview&quot;>
			&lt;div class = &quot;pmdynaform-file-resizeaudio-plus&quot;>
			&lt;span class=&quot;glyphicon glyphicon-picture&quot; style=&quot;margin:5px&quot;>&lt;/span>
			&lt;/div>
		&lt;/span>
		&lt;/buttonImage>	
	&lt;/div>	




	&lt;div class=&quot;pmdynaform-media-videoContainer&quot;>
	    &lt;video src=&quot;&lt;%-src%>&quot; preload=&quot;none&quot; controls=&quot;true&quot;>	       
			&lt;p>Your browser does not support the video tag.&lt;/p>
		&lt;/video>
			&lt;div class=&quot;pmdynaform-media-caption&quot;>&lt;/div>
			&lt;div class=&quot;pmdynaform-media-control&quot; display=&quot;none&quot;>
				&lt;div class=&quot;pmdynaform-media-btmControl&quot;>
					&lt;div class=&quot;btnPlay btn&quot; title=&quot;Play/Pause video&quot;>
					&lt;span class=&quot;glyphicon glyphicon-play&quot;>&lt;/span>
					&lt;/div>
					&lt;div class=&quot;pmdynaform-media-progress-bar&quot;>
						&lt;div class=&quot;pmdynaform-media-progress-external&quot;>
						&lt;/div>
						&lt;div class=&quot;pmdynaform-media-progress&quot;>
							&lt;span class=&quot;pmdynaform-media-bufferBar&quot;>&lt;/span>
							&lt;span class=&quot;pmdynaform-media-timeBar&quot;>&lt;/span>
						&lt;/div>
						&lt;div class=&quot;pmdynaform-media-progress-external&quot;>
						&lt;/div>
					&lt;/div>					
					&lt;div class=&quot;sound sound2 btn&quot; title=&quot;Mute/Unmute sound&quot;>
						&lt;span class=&quot;glyphicon glyphicon-volume-up&quot;>&lt;/span>&lt;/div>
					&lt;div class=&quot;btnFS btn&quot; title=&quot;Switch to full screen&quot;>
						&lt;span class=&quot;glyphicon glyphicon-fullscreen&quot;>&lt;/span>
					&lt;/div>
				&lt;/div>
				
			&lt;/div>
		&lt;/div>





	&lt;div class=&quot;pmdynaform-media-audioContainer&quot;>
	    &lt;audio preload=&quot;auto&quot; src=&quot;&lt;%-src%>&quot;>
	    	&lt;source src=&quot;&lt;%-src%>&quot; type=&quot;audio/mp3&quot;>  					
			&lt;source src=&quot;&lt;%-src%>&quot; type=&quot;audio/m4a&quot;>  								
	    	&lt;p>Your browser does not support the video tag.&lt;/p>
		&lt;/audio>
			&lt;div class=&quot;pmdynaform-media-caption&quot;>&lt;/div>
			&lt;div class=&quot;pmdynaform-media-control&quot;>
				&lt;div class=&quot;pmdynaform-media-btmControl&quot;>
					&lt;div class=&quot;btnPlay btn&quot; title=&quot;Play/Pause video&quot;>
						&lt;span class=&quot;glyphicon glyphicon-play&quot;>&lt;/span>&lt;/div>
					&lt;div class=&quot;pmdynaform-media-progress-bar&quot;>
						&lt;div class=&quot;pmdynaform-media-progress-external&quot;>
						&lt;/div>
						&lt;div class=&quot;pmdynaform-media-progress&quot;>
							&lt;span class=&quot;pmdynaform-media-bufferBar&quot;>&lt;/span>
							&lt;span class=&quot;pmdynaform-media-timeBar&quot;>&lt;/span>
						&lt;/div>
						&lt;div class=&quot;pmdynaform-media-progress-external&quot;>
						&lt;/div>						
					&lt;/div>					
					&lt;div class=&quot;sound sound2 btn&quot; title=&quot;Mute/Unmute sound&quot;>
						&lt;span class=&quot;glyphicon glyphicon-volume-up&quot;>&lt;/span>&lt;/div>
					&lt;div class=&quot;btnFS btn&quot; title=&quot;Switch to full screen&quot;>
						&lt;span class=&quot;glyphicon glyphicon-fullscreen&quot;>&lt;/span>
					&lt;/div>
				&lt;/div>
				
			&lt;/div>
		&lt;/div>




	&lt;%for(var i=0; i &lt; elements.length; i+=1){%>
		&lt;%if(type === 'imageMobile'){%>
			&lt;img src= &quot;data:image/png;base64,&lt;%-elements[i].base64%>&quot;
				class='img-thumbnail'
			 	alt='Thumbnail Image'>
		&lt;%}else{%>
			&lt;div class = 'container-element'>
				&lt;%if(ieVersion === 11 || ieVersion === 12){%>
					&lt;a href= &quot;&lt;%-elements[i].downloadLink%>&quot; class = &quot;multimedia-open fa fa-download fa-2&quot; aria-hidden='true'>&lt;/a>
				&lt;%}%>
				&lt;%if (type === &quot;videoMobile&quot;){%>
					&lt;video type='video/mp4'
				&lt;%}else{%>
					&lt;audio
				&lt;%}%>
					class= &quot;multimedia-web col-sm-12 col-md-12 col-lg-12&quot;
					id=&quot;&lt;%-elements[i].id%>&quot;
					src= &quot;&lt;%-elements[i].filePath%>&quot; controls
				>
			&lt;/div>
		&lt;%}%>
	&lt;%}%>






	&lt;div id=&quot;&lt;%=id%>&quot; class=&quot;&lt;%=namespace%>-field-&lt;%=type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%=colSpan%> col-md-&lt;%=colSpan%> col-lg-&lt;%=colSpan%> &lt;%}%> &lt;%=namespace%>-&lt;%=mode%>-&lt;%=type%> pmdynaform-field&quot;>
	    &lt;% if(group === &quot;form&quot;) {%>
		    &lt;label class=&quot;col-lg-&lt;%=colSpanLabel%> control-label pmdynaform-label&quot;> 
		    	&lt;span class=&quot;textlabel&quot;>&lt;%= label %>&lt;/span>
		        &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
		    &lt;/label>
		&lt;%} else if((group !== &quot;form&quot;) &amp;&amp; (layout === &quot;form&quot;)){%>
			&lt;label class=&quot;hidden-lg hidden-md hidden-sm visible-xs control-label pmdynaform-label col-xs-&lt;%=colSpanLabel%>&quot;>  
		  		&lt;span>&lt;%= label %>&lt;/span>
		  	&lt;/label>
		&lt;%}%>
		&lt;div &lt;%if(group === 'form'){ %> class=&quot;pmdynaform-file-control col-lg-&lt;%=colSpanControl%>&quot; &lt;%}%> >
			&lt;div class=&quot;pmdynaform-geo-container&quot;>
				&lt;button type=&quot;button&quot; class=&quot;btn btn-default btn-md&quot;>
				  	&lt;span class=&quot;glyphicon glyphicon-map-marker&quot;>&lt;/span> &lt;%=labelButton%>
				&lt;/button>				
				&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
					&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%=hint%>&quot;>&lt;/span>
				&lt;%}%>								
				&lt;%if(preview) {%>
					&lt;div class=&quot;pmdynaform-ext-geo&quot; style=&quot;height:&lt;%=height%>;&quot;>&lt;/div>
				&lt;% } %> 
			&lt;/div>
		&lt;/div>
	&lt;/div>


	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
		&lt;% if(group === &quot;form&quot;) {%>
			&lt;label class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
	        	&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
	            &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
	        &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if (group === 'form'){ %> 
		  			class=&quot;col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
		  		&lt;%} else if (layout === &quot;responsive&quot;) {%>
		  			class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
		  		&lt;%}%>>
		  		&lt;scanner id=&quot;PMD-&lt;%-id%>&quot;>
				&lt;div class=&quot;pmdynaform-label-options&quot;>
							&lt;span>&lt;%-value%>&lt;/span>							
				&lt;/div>
				&lt;/scanner>
		  		&lt;button type=&quot;button&quot; class=&quot;btn btn-default btn-md&quot;>
				  	&lt;span class=&quot;glyphicon glyphicon-qrcode&quot;>&lt;/span> &lt;%-labelButton%>
				&lt;/button>
				
			&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
				&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
			&lt;%}%>
		&lt;/div>
	&lt;/div>




	&lt;div class=&quot;pmdynaform-file-containerimage&quot;>
		&lt;buttonImage>
		&lt;span class= &quot;pmdynaform-file-preview&quot;>
			&lt;div class = &quot;pmdynaform-file-resizeimage-plus&quot;>+
			&lt;/div>
		&lt;/span>
		&lt;/buttonImage>
	&lt;/div>



	&lt;span>
		&lt;%-label%>
	&lt;/span>



	&lt;div id=&quot;&lt;%-id%>&quot; class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%> &lt;%}%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
	    &lt;% if(group === &quot;form&quot;) {%>
		    &lt;label class=&quot;col-sm-&lt;%-colSpanLabel%> col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
		    	&lt;span class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
		        &lt;%if(required){%>
		  			&lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
		  		&lt;%}%>
		    &lt;/label>
		&lt;%}%>
		&lt;div &lt;%if(group === 'form'){ %> class=&quot;pmdynaform-file-control col-sm-&lt;%-colSpanControl%> col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%>  pmdynaform-field-control&quot; &lt;%}%> >
			&lt;div id=&quot;PMD-&lt;%-id%>&quot; class=&quot;pmdynaform-signature-container&quot;>
				&lt;button type=&quot;button&quot; class=&quot;btn btn-default btn-md&quot;>
				  	&lt;span class=&quot;glyphicon glyphicon-edit&quot;>&lt;/span>&lt;%-labelButton%>&lt;/button>
				&lt;%if (hint !== &quot;&quot; &amp;&amp; hint !== null){%>
					&lt;span class=&quot;glyphicon glyphicon-info-sign&quot; data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-hint%>&quot;>&lt;/span>
				&lt;%}%>								
				&lt;%if(preview) {%>
					&lt;div class=&quot;pmdynaform-ext-signature&quot; style=&quot;height:&lt;%-height%>;&quot;>&lt;/div>
				&lt;% } %> 
			&lt;/div>
		&lt;/div>
	&lt;/div>


    &lt;div
            id=&quot;&lt;%-id%>&quot;
            name=&quot;field-&lt;%-name%>&quot;
            class=&quot;&lt;%-namespace%>-field-&lt;%-type%>&lt;%if (group === 'form'){%>form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%>&lt;%}%>&lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> pmdynaform-field&quot;>
        &lt;%if(group === &quot;form&quot;) {%>
        &lt;label
            for=&quot;&lt;%-name%>&quot;
            class=&quot;col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
            &lt;span
                class=&quot;textlabel&quot;>
                    &lt;%-label%>
            &lt;/span>
            &lt;%if(required){%>
            &lt;span
                    class=&quot;pmdynaform-field-required&quot;>*
            &lt;/span>
            &lt;%}%>
        &lt;/label>
        &lt;%}%>
        &lt;%if(layout === &quot;form&quot;){%>
        &lt;label
                for=&quot;&lt;%-name%>&quot;
                class=&quot;hidden-lg hidden-md hidden-sm visible-xs control-label pmdynaform-label col-xs-&lt;%-colSpanLabel%>&quot;>
            &lt;span
                    class=&quot;textlabel&quot;>
                &lt;%-label%>
            &lt;/span>
        &lt;/label>
        &lt;%}%>
        &lt;div
        &lt;%if(layout === &quot;form&quot;){%>
        class=&quot;col-xs-&lt;%-colSpanControl%> col-sm-12 col-md-12 col-lg-12&quot;
        &lt;%}else if (layout === &quot;responsive&quot;){%>
        class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;
        &lt;%}else{%>
        class=&quot;col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
        &lt;%}%>
        >
        &lt;div class=&quot;pmdynaform-control-checkbox-list form-control&quot;>
            &lt;div class=&quot;pmdynaform-checkbox-item field-container&quot;>
            &lt;!--here append to control tag-->
            &lt;/div>
            &lt;%if(group===&quot;form&quot; &amp;&amp; hint){%>
                    &lt;span
                            class=&quot;glyphicon glyphicon-info-sign&quot;
                            data-toggle=&quot;tooltip&quot;
                            data-placement=&quot;bottom&quot;
                            title=&quot;&lt;%-hint%>&quot;>
                    &lt;/span>
            &lt;%}%>
        &lt;/div>
        &lt;input
            type = &quot;hidden&quot;
            id=&quot;&lt;%-nameToPostLabelControl%>&quot;
            name=&quot;&lt;%-nameToPostLabelControl%>&quot;
            value = &quot;&lt;%-data['label']%>&quot;
        >
    &lt;/div>
    &lt;/div>


    &lt;input
        type =&quot;&lt;%-type%>&quot;
        id =&quot;&lt;%if(activePostForm){%>&lt;%-nameToPostControl%>&lt;%}else{%>&lt;%-id%>&lt;%}%>&quot;
        class = &quot;&lt;%-namespace%>-control-&lt;%-type%>&quot;
        name =&quot;&lt;%if(activePostForm){%>&lt;%-nameToPostControl%>&lt;%}else{%>&lt;%-name%>&lt;%}%>&quot;
        value =  &quot;&lt;%-value%>&quot;
    &lt;%if(disabled){%>
        disabled
    &lt;%}%>
    &lt;%if(selected){%>
        checked
    &lt;%}%>
    >
    &lt;span class=&quot;cloneUnchecked-container&quot;>&lt;/span>


	&lt;div class='alert alert-&lt;%-type%> fade in'>
		&lt;%if(closable){%>
			&lt;a href='#' class='close' data-dismiss='alert'>&lt;/a>
		&lt;%}%>
		&lt;strong>&lt;%-emphasisMessage%>&lt;/strong>
		&lt;%-message%>
	&lt;/div>


    &lt;div class=&quot;modal fade pm-modal-global&quot; id=&quot;modalProgressBar&quot; tabindex=&quot;-1&quot; role=&quot;dialog&quot;>
        &lt;div class=&quot;modal-dialog modal-sm&quot;>
            &lt;div class=&quot;modal-content&quot;>
                &lt;div class=&quot;modal-body&quot;>
                    &lt;div class=&quot;pm-modal-loading&quot;>&lt;/div>
                &lt;/div>
            &lt;/div>
        &lt;/div>
    &lt;/div>



        &lt;div class=&quot;pm-progressbar col-xs-&lt;%-col%>&quot;>
            &lt;div class=&quot;progressbar-title&quot;>
                &lt;div> &lt;%-title%>&lt;/div>
            &lt;/div>
            &lt;div class=&quot;progress&quot;>
                &lt;div class=&quot;progress-bar progress-bar-&lt;%-type%>
                  &lt;%if(striped){%>
                        progress-bar-striped
                  &lt;%}%>
                  &lt;%if(animate){%>
                        active
                  &lt;%}%>
                 &quot; role=&quot;progressbar&quot; aria-valuenow=&quot;&lt;%-value%>&quot;
                     aria-valuemin=&quot;0&quot;
                     aria-valuemax=&quot;100&quot;
                     style=&quot;width: &lt;%-value%>%;&quot;>
                    &lt;%-value%>%
                &lt;/div>
            &lt;/div>
        &lt;/div>


    &lt;div id=&quot;&lt;%-id%>&quot; name=&quot;field-&lt;%-name%>&quot;
         class=&quot;&lt;%-namespace%>-field-&lt;%-type%> &lt;%-namespace%>-&lt;%-mode%>-&lt;%-type%> &lt;%if (group === 'form'){%> form-group col-sm-&lt;%-colSpan%> col-md-&lt;%-colSpan%> col-lg-&lt;%-colSpan%>  pmdynaform-field&lt;%}%>&quot;>
        &lt;div class=&quot;row&quot;>
            &lt;label for=&quot;&lt;%-name%>&quot;
                   class=&quot;col-md-&lt;%-colSpanLabel%> col-lg-&lt;%-colSpanLabel%> control-label pmdynaform-label&quot;>
                &lt;span data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; class=&quot;textlabel&quot;>&lt;%- label %>&lt;/span>
                &lt;%if(required){%>
                &lt;span class=&quot;pmdynaform-field-required&quot;>*&lt;/span>
                &lt;%}%>
            &lt;/label>

            &lt;div
            &lt;%if (group === 'form'){ %>
            class=&quot;col-md-&lt;%-colSpanControl%> col-lg-&lt;%-colSpanControl%> pmdynaform-field-control&quot;
            &lt;%}%>
            >
            &lt;div class=&quot;pmdynaform-multiplefile-control&quot;>
                &lt;%if (mode === 'disabled'){ %>
                &lt;button type=&quot;button&quot; class=&quot;btn btn-uploadfile-disabled&quot;>&lt;%- labelButton %>&lt;/button>
                &lt;%}else if(mode === 'edit') {%>
                &lt;button type=&quot;button&quot; class=&quot;btn btn-uploadfile&quot;>&lt;%- labelButton %>&lt;/button>
                &lt;%}%>
                &lt;input type=&quot;file&quot; multiple=&quot;multiple&quot; accept=&quot;&lt;%-extensions%>&quot; style=&quot;display:none&quot;>
            &lt;/div>
        &lt;/div>
    &lt;/div>
    &lt;div class=&quot;pmdynaform-multiplefile-box&quot;>&lt;/div>
    &lt;/div>



    &lt;div class=&quot;multiplefile-row row&quot;>
        &lt;div class=&quot;multiplefile-container col-xs-&lt;%-col%>&quot;>

        &lt;/div>
    &lt;/div>



    &lt;div class=&quot;multiplefile-icon col-xs-&lt;%-col%>&quot;>
           &lt;span class=&quot;multiplefile-icon&quot;>
               &lt;i class=&quot;fa fa-&lt;%-icon%>&quot; aria-hidden=&quot;true&quot;>&lt;/i>
           &lt;/span>
    &lt;/div>



    &lt;div class=&quot;multiplefile-buttons col-xs-&lt;%-col%>&quot;>
        &lt;div class=&quot;multiplefile-button-delete col-xs-&lt;%-colButton%>&quot;>
            &lt;i class=&quot;fa fa-trash&quot; aria-hidden=&quot;true&quot;>&lt;/i>
        &lt;/div>
        &lt;div class=&quot;multiplefile-button-download col-xs-&lt;%-colButton%>&quot;>
            &lt;a href=&quot;&lt;%-href%>&quot;>&lt;i class=&quot;fa fa-download&quot; aria-hidden=&quot;true&quot;>&lt;/i>&lt;a>
        &lt;/div>
        &lt;div class=&quot;multiplefile-button-pencil col-xs-&lt;%-colButton%>&quot; style=&quot;display: none&quot;>
            &lt;i class=&quot;fa fa-pencil&quot; aria-hidden=&quot;true&quot;>&lt;/i>
        &lt;/div>
    &lt;/div>



    &lt;div class=&quot;pm-multiplefile-download col-xs-&lt;%-col%>&quot;>
        &lt;div class=&quot;multiplefile-title&quot;>
            &lt;div> &lt;%-title%>&lt;/div>
        &lt;/div>
    &lt;/div>



    &lt;div class=&quot;pm-multiplefile-grid&quot;>
        &lt;div class=&quot;pm-multiplefile-grid-label&quot;>
            &lt;span class=&quot;multiple-file-grid-web&quot;  data-toggle=&quot;tooltip&quot; data-placement=&quot;top&quot; title=&quot;Tooltip on top&quot;>Choose File&lt;/span>

        &lt;/div>
        &lt;div class=&quot;pm-multiplefile-grid-icon&quot;>
            &lt;a class=&quot;pm-multiplefile-upload&quot;>&lt;i class=&quot;fa fa-upload&quot; aria-hidden=&quot;true&quot;>&lt;/i>&lt;/a>
        &lt;/div>
    &lt;/div>



    &lt;div class=&quot;pm-multiplefile-grid-label&quot;>
        &lt;ul>
            &lt;% for(var i = 0;i &lt; gridDetailArray.length; i += 1) { %>
                &lt;li class=&quot;multiple-file-grid-web&quot;  data-toggle=&quot;tooltip&quot; data-placement=&quot;bottom&quot; title=&quot;&lt;%-gridDetailArray[i]%>&quot;>&lt;%-gridDetailArray[i]%>&lt;/li>
            &lt;% } %>
        &lt;/ul>
    &lt;/div>
    &lt;div class=&quot;pm-multiplefile-grid-icon&quot;>
        &lt;a class=&quot;pm-multiplefile-upload&quot;>&lt;i class=&quot;fa fa-upload&quot; aria-hidden=&quot;true&quot;>&lt;/i>&lt;/a>
    &lt;/div>


    &lt;div class=&quot;modal fade&quot; id=&quot;modalUpload&quot; tabindex=&quot;-1&quot; role=&quot;dialog&quot;>
        &lt;div class=&quot;modal-dialog&quot;>
            &lt;div class=&quot;modal-content&quot;>
                &lt;div class=&quot;modal-header&quot;>
                    &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;modal&quot; aria-label=&quot;Close&quot;>
                        &lt;span aria-hidden=&quot;true&quot;>&amp;times;&lt;/span>
                    &lt;/button>
                    &lt;h4 class=&quot;modal-title&quot;>&amp;nbsp; &lt;/h4>
                &lt;/div>
                &lt;div class=&quot;modal-body&quot;>
                    &lt;div class=&quot;row&quot;>
                        &lt;div class=&quot;pm-modal-upload&quot;>
                            &lt;%if (mode === 'disabled'){ %>
                            &lt;button type=&quot;button&quot; class=&quot;btn btn-uploadfile-disabled&quot;>&lt;%- labelButton %>&lt;/button>
                            &lt;%}else if (mode === 'edit'){%>
                            &lt;button type=&quot;button&quot; class=&quot;btn btn-uploadfile&quot;>&lt;%- labelButton %>&lt;/button>
                            &lt;%}%>
                            &lt;input type=&quot;file&quot; multiple=&quot;multiple&quot; accept=&quot;&lt;%-extensions%>&quot; style=&quot;display:none&quot;>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;row&quot;>
                        &lt;div class=&quot;pmdynaform-file-box&quot;>&lt;/div>
                    &lt;/div>
                &lt;/div>
                &lt;div class=&quot;modal-footer&quot;>
                    &lt;button type=&quot;button&quot; class=&quot;btn btn-default&quot; data-dismiss=&quot;modal&quot;>&lt;%- labelClose %>&lt;/button>
                &lt;/div>
            &lt;/div>
        &lt;/div>
    &lt;/div>


        
        
        
var jsondata = {&quot;name&quot;:&quot;Customer Creation&quot;,&quot;description&quot;:&quot;Straumann Customer Creation&quot;,&quot;items&quot;:[{&quot;type&quot;:&quot;form&quot;,&quot;variable&quot;:&quot;&quot;,&quot;var_uid&quot;:&quot;&quot;,&quot;dataType&quot;:&quot;&quot;,&quot;id&quot;:&quot;3963972475b0d0fcc87c5d6018612178&quot;,&quot;name&quot;:&quot;Customer Creation&quot;,&quot;description&quot;:&quot;Straumann Customer Creation&quot;,&quot;mode&quot;:&quot;edit&quot;,&quot;script&quot;:{&quot;type&quot;:&quot;js&quot;,&quot;code&quot;:&quot;$(\&quot;#grid_lictype\&quot;).hideColumn(3);\n$(\&quot;#ob_division_id\&quot;).hide();\n\/\/alert($(\&quot;#ob_division_id\&quot;).getValue());\n$(\&quot;#pm_org_id\&quot;).hide();\n$(\&quot;#pm_org\&quot;).hide();\n$(\&quot;#ob_branch_id\&quot;).hide();\n$(\&quot;#ob_branch\&quot;).hide();\n$(\&quot;#login_username\&quot;).hide();\n$(\&quot;#login_usermail\&quot;).hide();\n$(\&quot;#gstunregcust\&quot;).hide();\n\/\/$(\&quot;#cst_no\&quot;).hide();\n$(\&quot;#LoginName\&quot;).hide();\n$(\&quot;#payment_terms\&quot;).hide();\n$(\&quot;#referenceNo\&quot;).hide();\n\n\nif($(\&quot;#is_gst_register\&quot;).getValue() == 1)\n{  \n  $(\&quot;#grid_gst\&quot;).show();\n  $(\&quot;#gstunregcust\&quot;).hide();\n    \/\/$(\&quot;#cst_no\&quot;).hide();\n  \/\/$(\&quot;#vat_tin_no\&quot;).hide();\n  \n}else{\n    $(\&quot;#gstunregcust\&quot;).show();\n    $(\&quot;#grid_gst\&quot;).hide();  \n    \/\/$(\&quot;#cst_no\&quot;).show();\n     \/\/$(\&quot;#vat_tin_no\&quot;).show();\n  }\n                  \n                  \n\n\n\n\n\/\/alert(\&quot;OK\&quot;);\n\/\/ Set value of cust_category which is in header and set in ln_cust_category_id which is in Licence Info Grid\n$(\&quot;#cust_category\&quot;).setOnchange(setCustId);\nfunction setCustId() {\n  \n  var rows = $(\&quot;#grid_lictype\&quot;).getNumberRows();\n  \n   for(var i=1;i&lt;=rows;i++)\n   {\n     \n\t  $(\&quot;#grid_lictype\&quot;).setValue($(\&quot;#cust_category\&quot;).getValue(), i, 3);\n    \n    }\n  \n}\n\n\n\/\/at the time of adding row  cust_acategory will set\n$(\&quot;#grid_lictype\&quot;).onAddRow(function(aNewRow, oGrid, rowIndex) {\n       \n  \t\taNewRow[2].setValue($(\&quot;#cust_category\&quot;).getValue());  \n  \n\n})\n\n\n$(\&quot;#grid_lictype\&quot;).onShowRowDialog(function(row, gridObject, index, rowModel) { \n        \n  \t\trow[2].setValue($(\&quot;#cust_category\&quot;).getValue());   \n  \t\t   \n})\n\n\n\n\/\/To uppercase pan no.\nfunction panno()\n{\n  \tvar str = $(\&quot;#pan_no\&quot;).getValue();\n    \/\/alert(str);\n    $(\&quot;#pan_no\&quot;).setValue(str.replace(\/\\w\\S*\/g, function(txt){return txt.toUpperCase();}));\n       var rows = $(\&quot;#grid_gst\&quot;).getNumberRows();\n\t   \/\/alert(rows);\n      \/\/at the time of adding row  Header PAN NO  will set in GSTIN grid when gst is checked\n        for (var i = 1; i &lt;= rows; i++) {\n          $(\&quot;#grid_gst\&quot;).setValue($(\&quot;#pan_no\&quot;).getValue(), i, 5);\n        }\n}\n\n\n$('#pan_no').setOnchange(panno);\n\n\/\/To uppercase Licence No.\n\n$(\&quot;#submit0000000001\&quot;).find(\&quot;button\&quot;).on(\&quot;click\&quot; , function() {\n \n  \n  var rows = $(\&quot;#grid_lictype\&quot;).getNumberRows();\n \n  \n  for(var i=1;i&lt;=rows;i++)\n    \n   {\n     \/\/TO uppercase license No. for mobile\n     var licno = $(\&quot;#grid_lictype\&quot;).getValue(i,2);\n     $(\&quot;#grid_lictype\&quot;).setValue(licno.replace(\/\\w\\S*\/g, function(txt){return txt.toUpperCase();}),i,2);\n     \n     \n   }\n})\n\/\/Grid Hide and show based on GST is Register Check Box.\n$(\&quot;#is_gst_register\&quot;).setOnchange(setGstGrid);\nfunction setGstGrid() { \n  \nif($(\&quot;#is_gst_register\&quot;).getValue() == 1)\n{\n  \n  $(\&quot;#grid_gst\&quot;).show();\n  $(\&quot;#gstunregcust\&quot;).hide();\n    \/\/$(\&quot;#cst_no\&quot;).hide();\n  \/\/$(\&quot;#vat_tin_no\&quot;).hide();\n   \/\/$(\&quot;#cst_no\&quot;).getControl().attr('Required', false); \n    \/\/$(\&quot;#vat_tin_no\&quot;).getControl().attr('Required', false);  \n  \n}\n\n  else{    \n    $(\&quot;#grid_gst\&quot;).hide();\n    $(\&quot;#gstunregcust\&quot;).show();\n     var rows1 = $(\&quot;#grid_gst\&quot;).getNumberRows();\n  \n    for (var i= rows1; i >= 1; i--) {\n   \t\t$(\&quot;#grid_gst\&quot;).deleteRow(i);\n   \t\t\/\/The first row can't be deleted, so clear the fields in the first row:\n     \t\/*for (var j = 1; j &lt; 6; j++) {\n     \t  jQuery(\&quot;#grid_lictype\&quot;).setValue(\&quot;\&quot;,j,1);\n          jQuery(\&quot;#grid_lictype\&quot;).setValue(\&quot;\&quot;,j,2);\n          jQuery(\&quot;#grid_lictype\&quot;).setValue(\&quot;\&quot;,j,3);\n          jQuery(\&quot;#grid_lictype\&quot;).setValue(\&quot;\&quot;,j,4);\n          jQuery(\&quot;#grid_lictype\&quot;).setValue(\&quot;\&quot;,j,5);\n     \t\t\/\/$(\&quot;#gridTransact\&quot;).setText(\&quot;\&quot;, 1, j);\n     \t\t\/\/getFieldById(\&quot;gridTransact\&quot;).setValue(\&quot;\&quot;, 1, j);\n   \t\t}*\/\n   }\n    \/\/$(\&quot;#cst_no\&quot;).show();\n    \/\/ $(\&quot;#vat_tin_no\&quot;).show();\n    \/\/$(\&quot;#cst_no\&quot;).getControl().attr('Required', true); \n    \/\/$(\&quot;#vat_tin_no\&quot;).getControl().attr('Required', true); \n    \/\/return false;\n  }\n}\n\n\/\/on save button--it will check GST Register is check or not if check then all grids of GSTIN is mendatory\n$(\&quot;#submit0000000001\&quot;).find(\&quot;button\&quot;).on(\&quot;click\&quot; , function() {\n\n  var rows = $(\&quot;#grid_gst\&quot;).getNumberRows();\n   \/\/ this variable for comparing pan no to below gst grid\n  \/\/var panno=$(\&quot;#pan_no\&quot;).getValue();\n \n  \n  if($(\&quot;#is_gst_register\&quot;).getValue() == 1)\n{\n  for(var i=1;i&lt;=rows;i++)\n    \n   {\n     \n     \/\/GST Related Validation\n     var statecode =$(\&quot;#grid_gst\&quot;).getText(i,3);\n     \n     var gstinno =$(\&quot;#grid_gst\&quot;).getValue(i,4);\n     \n     \/\/alert(gstinno);\n       \n     \n     var gstno = $(\&quot;#grid_gst\&quot;).getValue(i,4);\n    \/\/Uppercase GST Unique ID\n     $(\&quot;#grid_gst\&quot;).setValue(gstno.replace(\/\\w\\S*\/g, function(txt){return txt.toUpperCase();}),i,4);\n     \n     if($(\&quot;#grid_gst\&quot;).getValue(i,1 ) == \&quot;\&quot; || $(\&quot;#grid_gst\&quot;).getValue(i,2 ) == \&quot;\&quot; || $(\&quot;#grid_gst\&quot;).getValue(i,4 ) == \&quot;\&quot;){\n       \n       \/\/alert(\&quot;Please Enter Registerd State\&quot;);\n       \/\/alert(\&quot;OK\&quot;);\n       \n        window.dynaform.flashMessage( {\n       duration : 8000,\n       emphasisMessage: \&quot;Warning:\&quot;,\n       message:\&quot;All fields are mendatory when GST Register is Checked,Please enter all GSTIN fields\&quot;,\n       type : 'danger',\n       appendTo:$(\&quot;#grid_gst\&quot;),\n       \/\/absoluteTop : true   \n        \n    } );\n      \n       \n       return false;\n    \n       }\n     \n     \n     var n = gstinno.startsWith(statecode);\n  \n     \/\/alert(n);\n     \n     if(n == false){\n       \n     alert(\&quot;First 2 digit of GSTIN Unique Code should be same as State Code\&quot;);\n     return false;\n     \n     }\n     \n     \/\/return false;\n     var panno=$(\&quot;#pan_no\&quot;).getValue();\n     \n     \/\/alert(panno);\n    \/\/ var str = \&quot;Hello world!\&quot;;\n     var gstinno_panno = gstinno.substring(2, 12);\n     \/\/alert(gstinno_panno);\n     \/\/return false;\n     \n     if(panno !== gstinno_panno){\n       \n       alert(\&quot;PAN No. should match 10 digit of GSTIN Unique Code after the first 2 digit of State Code\&quot;);\n       \n       return false;\n       \n       }\n       \n   }\n}\n  \n})\n\n\/\/at the time of adding row  Header PAN NO  will set in GSTIN grid\n$(\&quot;#grid_gst\&quot;).onAddRow(function(aNewRow, oGrid, rowIndex) {\n\n    aNewRow[4].setValue($(\&quot;#pan_no\&quot;).getValue());\n\n\n})\n\n\n\n&quot;},&quot;language&quot;:&quot;en&quot;,&quot;externalLibs&quot;:&quot;&quot;,&quot;printable&quot;:false,&quot;items&quot;:[[{&quot;type&quot;:&quot;title&quot;,&quot;id&quot;:&quot;title0000000001&quot;,&quot;label&quot;:&quot;New Customer Creation&quot;,&quot;colSpan&quot;:12}],[{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;ob_division&quot;,&quot;var_uid&quot;:&quot;816362646592fc38779fba8048951611&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;ob_division&quot;,&quot;name&quot;:&quot;ob_division&quot;,&quot;label&quot;:&quot;Activity:&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:true,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;&quot;,&quot;validateMessage&quot;:&quot;&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;disabled&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;ob_division&quot;,&quot;colSpan&quot;:8,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;Pan Health&quot;,&quot;label&quot;:&quot;Pan Health&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;referenceNo&quot;,&quot;var_uid&quot;:&quot;8418889135b0d171aee1e65040409432&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;referenceNo&quot;,&quot;name&quot;:&quot;referenceNo&quot;,&quot;label&quot;:&quot;Reference No.&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;&quot;,&quot;validateMessage&quot;:&quot;&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;view&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;referenceNo&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}}],[{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;cust&quot;,&quot;var_uid&quot;:&quot;4412005615934e8ad309d07068764818&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;cust&quot;,&quot;name&quot;:&quot;cust&quot;,&quot;label&quot;:&quot;Customer:&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;Enter Customer Name&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:true,&quot;textTransform&quot;:&quot;upper&quot;,&quot;validate&quot;:&quot;[A-Za-z]+$&quot;,&quot;validateMessage&quot;:&quot;Enter alphabets only.&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;cust&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;dropdown&quot;,&quot;variable&quot;:&quot;cust_category&quot;,&quot;var_uid&quot;:&quot;3975726985934e90f29b104027299264&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;cust_category&quot;,&quot;name&quot;:&quot;cust_category&quot;,&quot;label&quot;:&quot;Business Partner Category:&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;---Select Business Partner Category---&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:true,&quot;mode&quot;:&quot;parent&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;7087127375b0e43abaceb90059159408&quot;,&quot;dbConnectionLabel&quot;:&quot;[192.168.2.224:7326] pgsql: prism_db&quot;,&quot;sql&quot;:&quot;select C_BP_Group_id,name from C_BP_Group where \nad_org_id='@=ob_division_id'\norder by 2&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;options&quot;:[],&quot;var_name&quot;:&quot;cust_category&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;pan_no&quot;,&quot;var_uid&quot;:&quot;6084899785934e9a5d98415033581787&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;pan_no&quot;,&quot;name&quot;:&quot;pan_no&quot;,&quot;label&quot;:&quot;PAN No.:&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;Enter PAN No.&quot;,&quot;hint&quot;:&quot;XXXXX9999X where X is a alphabatic character and 9 is a numeric digit.&quot;,&quot;required&quot;:true,&quot;textTransform&quot;:&quot;upper&quot;,&quot;validate&quot;:&quot;^[A-Za-z]{5}\\d{4}[A-Za-z]{1}$&quot;,&quot;validateMessage&quot;:&quot;Invalid Pan No.&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;pan_no&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}}],[{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;email&quot;,&quot;var_uid&quot;:&quot;5428460295934e9c3d92ac6098657406&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;email&quot;,&quot;name&quot;:&quot;email&quot;,&quot;label&quot;:&quot;Email:&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;Enter Email Id&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;^([\\w-\\.]+)@((\\[[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.)|(([\\w-]+\\.)+))([a-zA-Z]{2,4}|[0-9]{1,3})(\\]?)$&quot;,&quot;validateMessage&quot;:&quot;Invalid Email Id&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;email&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;contact_person&quot;,&quot;var_uid&quot;:&quot;3007680175934ebe6c2fae3006256794&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;contact_person&quot;,&quot;name&quot;:&quot;contact_person&quot;,&quot;label&quot;:&quot;Contact Person Name&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;Enter Contact Person Name&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;[A-Za-z]+$&quot;,&quot;validateMessage&quot;:&quot;Enter alphabets only.&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;contact_person&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;dropdown&quot;,&quot;variable&quot;:&quot;priceList&quot;,&quot;var_uid&quot;:&quot;5924902295b0d1d71bcd976053376731&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;priceList&quot;,&quot;name&quot;:&quot;priceList&quot;,&quot;label&quot;:&quot;Price List&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;---Select Price List---&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:true,&quot;mode&quot;:&quot;parent&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;7087127375b0e43abaceb90059159408&quot;,&quot;dbConnectionLabel&quot;:&quot;[192.168.2.224:7326] pgsql: prism_db&quot;,&quot;sql&quot;:&quot;select m_pricelist_id,name from m_pricelist where ad_org_id = '@=ob_division_id'\n and IsSOPriceList='Y' &quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;options&quot;:[],&quot;var_name&quot;:&quot;priceList&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}}],[{&quot;type&quot;:&quot;suggest&quot;,&quot;variable&quot;:&quot;sales_person&quot;,&quot;var_uid&quot;:&quot;1836434185934ea52256776081248543&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;sales_person&quot;,&quot;name&quot;:&quot;sales_person&quot;,&quot;label&quot;:&quot;Sales Person:&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:true,&quot;mode&quot;:&quot;parent&quot;,&quot;datasource&quot;:&quot;dataVariable&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:[[&quot;8033D097D2D84BE99F7580CCF7E5E4D6&quot;,&quot;CEO\/001&quot;],[&quot;EE8747ADB5AC454892BA7D79FF978940&quot;,&quot;E\/ASM\/001&quot;],[&quot;84450A5D1BDD463A9E5B9AF1E51BC6C3&quot;,&quot;E\/RSM\/001&quot;],[&quot;E82BFF15D81143B1A88FBAF3271CD903&quot;,&quot;E\/WB\/001&quot;],[&quot;F166BBE13516402E8FF8477117022944&quot;,&quot;E\/WB\/002&quot;],[&quot;906D68B8D21342AEA954E8FC86F4EB06&quot;,&quot;GM\/001&quot;],[&quot;16E2E5455F5949249CF05792C528D495&quot;,&quot;GM\/002&quot;],[&quot;7CE93F8E01F24883A3D34972782CBE0E&quot;,&quot;N\/ASM\/001&quot;],[&quot;3705205481804396A6E4AA5C488DBF72&quot;,&quot;N\/ASM\/002&quot;],[&quot;CC298D6D2A50463E952FD99E84EC099A&quot;,&quot;N\/PUN\/001&quot;],[&quot;1241A53E545A48E4A16F0F8A47B8445F&quot;,&quot;N\/PUN\/002&quot;],[&quot;35A7884D17E34CA3884E05333BDCF4C5&quot;,&quot;N\/RSM\/001&quot;],[&quot;6348A6DA3DB4480D99CFA8050C06F2C6&quot;,&quot;N\/RSM\/002&quot;],[&quot;43A6701FCC6A4511AA1B9FC5B71A9D43&quot;,&quot;NSM\/001&quot;],[&quot;F4DC2376159A44F08594F5EDF437C6AC&quot;,&quot;VP\/001&quot;],[&quot;3B04D556061E44E782E5D9C5793B02CF&quot;,&quot;W\/ASM\/001&quot;],[&quot;27E611DB08C044048378761A9BADF1D6&quot;,&quot;W\/ASM\/002&quot;],[&quot;3B1B5EBE29B84A15B6995E198A9A8D97&quot;,&quot;W\/MUM\/001&quot;],[&quot;FB2AEAF802A040F986D3C6FAF7581C02&quot;,&quot;W\/MUM\/002&quot;],[&quot;6EB7490870A94D3AA5F8F55D57F5652A&quot;,&quot;W\/MUM\/003&quot;],[&quot;C5D45EDC4DB740318614454B72F39E83&quot;,&quot;W\/RSM\/001&quot;]],&quot;options&quot;:[],&quot;delay&quot;:0,&quot;var_name&quot;:&quot;sales_person&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;colSpan&quot;:4},{&quot;colSpan&quot;:4}],[{&quot;type&quot;:&quot;checkbox&quot;,&quot;variable&quot;:&quot;is_gst_register&quot;,&quot;var_uid&quot;:&quot;633668533594e24223901e8002171276&quot;,&quot;dataType&quot;:&quot;boolean&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;is_gst_register&quot;,&quot;name&quot;:&quot;is_gst_register&quot;,&quot;label&quot;:&quot;GST Register:&quot;,&quot;defaultValue&quot;:true,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;mode&quot;:&quot;parent&quot;,&quot;options&quot;:[{&quot;value&quot;:&quot;1&quot;,&quot;label&quot;:&quot;Yes&quot;},{&quot;value&quot;:&quot;0&quot;,&quot;label&quot;:&quot;No&quot;}],&quot;var_name&quot;:&quot;is_gst_register&quot;,&quot;colSpan&quot;:4,&quot;data&quot;:{&quot;value&quot;:true,&quot;label&quot;:&quot;No&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;gstunregcust&quot;,&quot;var_uid&quot;:&quot;60511090559c33f9de12509062812220&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;gstunregcust&quot;,&quot;name&quot;:&quot;gstunregcust&quot;,&quot;label&quot;:&quot;GSTIN:&quot;,&quot;defaultValue&quot;:&quot;Unregistered Customer&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;&quot;,&quot;validateMessage&quot;:&quot;&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;disabled&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;gstunregcust&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;Unregistered Customer&quot;,&quot;label&quot;:&quot;Unregistered Customer&quot;}},{&quot;colSpan&quot;:4}],[{&quot;type&quot;:&quot;grid&quot;,&quot;variable&quot;:&quot;grid_gst&quot;,&quot;var_uid&quot;:&quot;690465530595c78973037f6094118293&quot;,&quot;dataType&quot;:&quot;grid&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;grid_gst&quot;,&quot;name&quot;:&quot;grid_gst&quot;,&quot;label&quot;:&quot;GSTIN&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;columns&quot;:[{&quot;type&quot;:&quot;suggest&quot;,&quot;variable&quot;:&quot;&quot;,&quot;var_uid&quot;:&quot;&quot;,&quot;dataType&quot;:&quot;&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;registerd_state&quot;,&quot;name&quot;:&quot;registerd_state&quot;,&quot;label&quot;:&quot;Registered State&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;mode&quot;:&quot;parent&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;1212240735931517bbd2707001586592&quot;,&quot;dbConnectionLabel&quot;:&quot;[10.0.0.42:7326] pgsql: prism_db&quot;,&quot;sql&quot;:&quot;select c_region_id,name from c_region  \nwhere c_country_id=(select c_country_id from c_country where name='India' and countrycode='IN') order by 2&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;options&quot;:[],&quot;delay&quot;:0,&quot;columnWidth&quot;:&quot;200&quot;,&quot;width&quot;:100,&quot;title&quot;:&quot;Registered State&quot;,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;&quot;,&quot;var_uid&quot;:&quot;&quot;,&quot;dataType&quot;:&quot;&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;tax_payer_name&quot;,&quot;name&quot;:&quot;tax_payer_name&quot;,&quot;label&quot;:&quot;Tax Payer Name&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;Enter Tax Payer Name&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;[A-Za-z]+$&quot;,&quot;validateMessage&quot;:&quot;Enter alphabets only.&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;columnWidth&quot;:&quot;200&quot;,&quot;width&quot;:100,&quot;title&quot;:&quot;Tax Payer Name&quot;,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;dropdown&quot;,&quot;variable&quot;:&quot;&quot;,&quot;var_uid&quot;:&quot;&quot;,&quot;dataType&quot;:&quot;&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;state_code&quot;,&quot;name&quot;:&quot;state_code&quot;,&quot;label&quot;:&quot;State Code&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;mode&quot;:&quot;disabled&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;1212240735931517bbd2707001586592&quot;,&quot;dbConnectionLabel&quot;:&quot;[10.0.0.42:7326] pgsql: prism_db&quot;,&quot;sql&quot;:&quot;select c_region_id, em_ingst_statecode from c_region  \nwhere c_country_id =(select c_country_id from c_country where name='India' and countrycode='IN') and c_region_id='@=registerd_state' order by 2\n&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;options&quot;:[],&quot;columnWidth&quot;:&quot;200&quot;,&quot;width&quot;:100,&quot;title&quot;:&quot;State Code&quot;,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;&quot;,&quot;var_uid&quot;:&quot;&quot;,&quot;dataType&quot;:&quot;&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;gstin_no&quot;,&quot;name&quot;:&quot;gstin_no&quot;,&quot;label&quot;:&quot;GST Unique ID&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;Enter GST Unique ID.&quot;,&quot;hint&quot;:&quot;It's a combination of all 5\n1.First 2 digit of Sate \n2.PAN No.\n3.1 Digit Numeric\n4.By Default Z character\n5.1 digit Alphanumeric&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;upper&quot;,&quot;validate&quot;:&quot;^[0-9]{2}[A-Z]{5}[0-9]{4}[A-Z]{1}[0-9]{1}Z[0-9A-Z]{1}$&quot;,&quot;validateMessage&quot;:&quot;Invalid GST Unique ID.\nIt Should be 15 Length.&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;columnWidth&quot;:&quot;200&quot;,&quot;width&quot;:100,&quot;title&quot;:&quot;GST Unique ID&quot;,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;&quot;,&quot;var_uid&quot;:&quot;&quot;,&quot;dataType&quot;:&quot;&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;gridPANno&quot;,&quot;name&quot;:&quot;gridPANno&quot;,&quot;label&quot;:&quot;PAN No.&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;&quot;,&quot;validateMessage&quot;:&quot;&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;disabled&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;columnWidth&quot;:&quot;150&quot;,&quot;width&quot;:100,&quot;title&quot;:&quot;PAN No.&quot;,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}}],&quot;data&quot;:[],&quot;mode&quot;:&quot;parent&quot;,&quot;layout&quot;:&quot;static&quot;,&quot;pageSize&quot;:&quot;0&quot;,&quot;addRow&quot;:true,&quot;deleteRow&quot;:true,&quot;title&quot;:&quot;GSTIN&quot;,&quot;colSpan&quot;:12}],[{&quot;type&quot;:&quot;dropdown&quot;,&quot;variable&quot;:&quot;payment_terms&quot;,&quot;var_uid&quot;:&quot;3830724695b0d0fcc964da7075645389&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;payment_terms&quot;,&quot;name&quot;:&quot;payment_terms&quot;,&quot;label&quot;:&quot;payment_terms&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;mode&quot;:&quot;disabled&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;7087127375b0e43abaceb90059159408&quot;,&quot;dbConnectionLabel&quot;:&quot;[192.168.2.224:7326] pgsql: prism_db&quot;,&quot;sql&quot;:&quot;select C_PaymentTerm_id,name from C_PaymentTerm \nwhere ad_org_id='0' and isactive ='Y'  \nand C_PaymentTerm_id='2DBE340FD31A47F6BE2EA297926C9A13'\n and ad_client_id='B9B34306DF554F8CA008EFE2BA6A2625' \norder by replace(replace(replace(replace(replace(name,'Days',''),'days',''),'Day',''),'day',''),'d','')::int&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;options&quot;:[],&quot;var_name&quot;:&quot;payment_terms&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[{&quot;value&quot;:&quot;2DBE340FD31A47F6BE2EA297926C9A13&quot;,&quot;label&quot;:&quot;0 Day&quot;}],&quot;data&quot;:{&quot;value&quot;:&quot;&quot;,&quot;label&quot;:&quot;&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;ob_division_id&quot;,&quot;var_uid&quot;:&quot;656649134592fc386237b46058054630&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;ob_division_id&quot;,&quot;name&quot;:&quot;ob_division_id&quot;,&quot;label&quot;:&quot;ob_division_id&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;&quot;,&quot;validateMessage&quot;:&quot;&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;ob_division_id&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;D766A659B2DD4A49BF95F04D6E7EA1C5&quot;,&quot;label&quot;:&quot;D766A659B2DD4A49BF95F04D6E7EA1C5&quot;}},{&quot;type&quot;:&quot;text&quot;,&quot;variable&quot;:&quot;LoginName&quot;,&quot;var_uid&quot;:&quot;5672667755b0e94ec6b2ac3011609564&quot;,&quot;dataType&quot;:&quot;string&quot;,&quot;protectedValue&quot;:false,&quot;id&quot;:&quot;LoginName&quot;,&quot;name&quot;:&quot;LoginName&quot;,&quot;label&quot;:&quot;LoginName&quot;,&quot;defaultValue&quot;:&quot;&quot;,&quot;placeholder&quot;:&quot;&quot;,&quot;hint&quot;:&quot;&quot;,&quot;required&quot;:false,&quot;textTransform&quot;:&quot;none&quot;,&quot;validate&quot;:&quot;&quot;,&quot;validateMessage&quot;:&quot;&quot;,&quot;maxLength&quot;:1000,&quot;formula&quot;:&quot;&quot;,&quot;mode&quot;:&quot;parent&quot;,&quot;operation&quot;:&quot;&quot;,&quot;datasource&quot;:&quot;database&quot;,&quot;dbConnection&quot;:&quot;workflow&quot;,&quot;dbConnectionLabel&quot;:&quot;PM Database&quot;,&quot;sql&quot;:&quot;&quot;,&quot;dataVariable&quot;:&quot;&quot;,&quot;var_name&quot;:&quot;LoginName&quot;,&quot;colSpan&quot;:4,&quot;optionsSql&quot;:[],&quot;data&quot;:{&quot;value&quot;:&quot;wasm.001&quot;,&quot;label&quot;:&quot;wasm.001&quot;}}],[{&quot;type&quot;:&quot;submit&quot;,&quot;id&quot;:&quot;submit0000000001&quot;,&quot;name&quot;:&quot;submit0000000001&quot;,&quot;label&quot;:&quot;Save&quot;,&quot;colSpan&quot;:12}]],&quot;variables&quot;:[{&quot;var_uid&quot;:&quot;816362646592fc38779fba8048951611&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;ob_division&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;8418889135b0d171aee1e65040409432&quot;,&quot;prj_uid&quot;:&quot;1973931515b0d0fc317c9f2074706254&quot;,&quot;var_name&quot;:&quot;referenceNo&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;4412005615934e8ad309d07068764818&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;cust&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;3975726985934e90f29b104027299264&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;cust_category&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;6084899785934e9a5d98415033581787&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;pan_no&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;5428460295934e9c3d92ac6098657406&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;email&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;3007680175934ebe6c2fae3006256794&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;contact_person&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;5924902295b0d1d71bcd976053376731&quot;,&quot;prj_uid&quot;:&quot;1973931515b0d0fc317c9f2074706254&quot;,&quot;var_name&quot;:&quot;priceList&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;1836434185934ea52256776081248543&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;sales_person&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;633668533594e24223901e8002171276&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;is_gst_register&quot;,&quot;var_field_type&quot;:&quot;boolean&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;boolean&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[{\&quot;value\&quot;:\&quot;1\&quot;,\&quot;label\&quot;:\&quot;true\&quot;},{\&quot;value\&quot;:\&quot;0\&quot;,\&quot;label\&quot;:\&quot;false\&quot;}]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;60511090559c33f9de12509062812220&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;gstunregcust&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;3830724695b0d0fcc964da7075645389&quot;,&quot;prj_uid&quot;:&quot;1973931515b0d0fc317c9f2074706254&quot;,&quot;var_name&quot;:&quot;payment_terms&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;656649134592fc386237b46058054630&quot;,&quot;prj_uid&quot;:&quot;805185203592fc1d69c11b2075339600&quot;,&quot;var_name&quot;:&quot;ob_division_id&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;},{&quot;var_uid&quot;:&quot;5672667755b0e94ec6b2ac3011609564&quot;,&quot;prj_uid&quot;:&quot;1973931515b0d0fc317c9f2074706254&quot;,&quot;var_name&quot;:&quot;LoginName&quot;,&quot;var_field_type&quot;:&quot;string&quot;,&quot;var_field_size&quot;:10,&quot;var_label&quot;:&quot;string&quot;,&quot;var_dbconnection&quot;:&quot;workflow&quot;,&quot;var_dbconnection_label&quot;:&quot;PM Database&quot;,&quot;var_sql&quot;:&quot;&quot;,&quot;var_null&quot;:0,&quot;var_default&quot;:&quot;&quot;,&quot;var_accepted_values&quot;:&quot;[]&quot;,&quot;inp_doc_uid&quot;:&quot;&quot;}]}]};
var httpServerHostname = &quot;http://10.0.0.79&quot;;
var pm_run_outside_main_app = 'true';
var dyn_uid = '3963972475b0d0fcc87c5d6018612178';
var __DynaformName__ = '1973931515b0d0fc317c9f2074706254_3963972475b0d0fcc87c5d6018612178';
var app_uid = '1900114415b23b235a90928094720964';
var prj_uid = '1973931515b0d0fc317c9f2074706254';
var step_mode = 'EDIT';
var workspace = 'devtest';
var credentials = {&quot;accessToken&quot;:&quot;77ad30ba17798a4d04d86d1631c31d7be9c67f8d&quot;,&quot;expiresIn&quot;:&quot;86400&quot;,&quot;tokenType&quot;:&quot;bearer&quot;,&quot;scope&quot;:&quot;view_processes edit_processes *&quot;,&quot;refreshToken&quot;:&quot;1e324e56da724ca6a5bd2008ffb701bac0429e11&quot;,&quot;clientId&quot;:&quot;x-pm-local-client&quot;,&quot;clientSecret&quot;:&quot;179ad45c6ce2cb97cf1029e212046e81&quot;};
var filePost = null;
var fieldsRequired = null;
var triggerDebug = false;
var sysLang = 'en';
var isRTL = false;
var pathRTLCss = '/lib/pmdynaform/build/css/PMDynaform-rtl.css';
var delIndex = 1;




    
        Case #: 2409     Title: #2409
    


     
    
        
     
    
    Next Step    

    


	
	
	      
	
	
		
			
				New Customer Creation
			
		
	
	      
	
	
        
            
            	Activity:
	            
		  			*
		  		
            
		
		
			
				
				
					
					
				
				
					
				
		
    

	
		
		
			Reference No.
			
		
		
		
		
			
			
			
			
            
                
            
			
		
		
		
		
	
	
	      
	
	
        
            
            	Customer:
	            
		  			*
		  		
            
		
		
			
				
				
					
					
				
				
					
				
		
    

	
		
		
			Business Partner Category:
			
			*
			
		
		
		
		
	
	
	
	
	
		PAN Counter Stockist
	
	
	
		PAN Stockist
	
	
	
		PAN Super Stockist
	
	

		
		
		
		
	
	

	
        
            
            	PAN No.:
	            
		  			*
		  		
            
		
		
			
				
				
					
					
						
					
				
				
					
				
		
    
	      
	
	
        
            
            	Email:
	            
            
		
		
			
				
				
					
					
				
				
					
				
		
    

	
        
            
            	Contact Person Name
	            
            
		
		
			
				
				
					
					
				
				
					
				
		
    

	
		
		
			Price List
			
			*
			
		
		
		
		
	
	
	
	
	
		ST Outward Pricelist
	
	
	
		Super Stockist Pricelist
	
	
	
		Counter Stockist Pricelist
	
	
	
		Stockist Pricelist
	
	

		
		
		
		
	
	
	      
	
	
        
            
            	Sales Person:
	            
		  			*
		  		
            
		
		
			
				
					
				
				
					
						
					
					
				
				
		
    

	
	

	
	
	      
	
	
		
		
			GST Register:
			
		
		
		
		
			
				
				
				
					
				
			
			
				
				
					
				
			
			
		
		
		
		
	
	
	
	

	
        
            
            	GSTIN:
	            
            
		
		
			
				
				
					
					
				
				
					
				
		
    

	
	
	      
	
	
		
			GSTIN
				
				
			
			
				
					
					New
				

			
		
		
			
				Registered StateTax Payer NameState CodeGST Unique IDPAN No.
				
					1
	
        
		
			
				
					
				
				
				
		
    

	
        
		
			
				
				
				
					
				
		
    

	
		
		
		
		
		
		
		
		
	
	

	
        
		
			
				
				
				
					
				
		
    

	
        
		
			
				
				
				
					
				
		
    

					
					
				
			
		
	
	      
	
	
		
		
			payment_terms
			
		
		
		
		
		
		
		0 Day
		
		
		
		
		
		
	
	

	
        
            
            	ob_division_id
	            
            
		
		
			
				
				
					
					
				
				
					
				
		
    

	
        
            
            	LoginName
	            
            
		
		
			
				
				
					
					
				
				
					
				
		
    
	      
	
	
		
			 Save 
		
	
$(&quot;#grid_lictype&quot;).hideColumn(3);
$(&quot;#ob_division_id&quot;).hide();
//alert($(&quot;#ob_division_id&quot;).getValue());
$(&quot;#pm_org_id&quot;).hide();
$(&quot;#pm_org&quot;).hide();
$(&quot;#ob_branch_id&quot;).hide();
$(&quot;#ob_branch&quot;).hide();
$(&quot;#login_username&quot;).hide();
$(&quot;#login_usermail&quot;).hide();
$(&quot;#gstunregcust&quot;).hide();
//$(&quot;#cst_no&quot;).hide();
$(&quot;#LoginName&quot;).hide();
$(&quot;#payment_terms&quot;).hide();
$(&quot;#referenceNo&quot;).hide();


if($(&quot;#is_gst_register&quot;).getValue() == 1)
{  
  $(&quot;#grid_gst&quot;).show();
  $(&quot;#gstunregcust&quot;).hide();
    //$(&quot;#cst_no&quot;).hide();
  //$(&quot;#vat_tin_no&quot;).hide();
  
}else{
    $(&quot;#gstunregcust&quot;).show();
    $(&quot;#grid_gst&quot;).hide();  
    //$(&quot;#cst_no&quot;).show();
     //$(&quot;#vat_tin_no&quot;).show();
  }
                  
                  




//alert(&quot;OK&quot;);
// Set value of cust_category which is in header and set in ln_cust_category_id which is in Licence Info Grid
$(&quot;#cust_category&quot;).setOnchange(setCustId);
function setCustId() {
  
  var rows = $(&quot;#grid_lictype&quot;).getNumberRows();
  
   for(var i=1;i&lt;=rows;i++)
   {
     
	  $(&quot;#grid_lictype&quot;).setValue($(&quot;#cust_category&quot;).getValue(), i, 3);
    
    }
  
}


//at the time of adding row  cust_acategory will set
$(&quot;#grid_lictype&quot;).onAddRow(function(aNewRow, oGrid, rowIndex) {
       
  		aNewRow[2].setValue($(&quot;#cust_category&quot;).getValue());  
  

})


$(&quot;#grid_lictype&quot;).onShowRowDialog(function(row, gridObject, index, rowModel) { 
        
  		row[2].setValue($(&quot;#cust_category&quot;).getValue());   
  		   
})



//To uppercase pan no.
function panno()
{
  	var str = $(&quot;#pan_no&quot;).getValue();
    //alert(str);
    $(&quot;#pan_no&quot;).setValue(str.replace(/\w\S*/g, function(txt){return txt.toUpperCase();}));
       var rows = $(&quot;#grid_gst&quot;).getNumberRows();
	   //alert(rows);
      //at the time of adding row  Header PAN NO  will set in GSTIN grid when gst is checked
        for (var i = 1; i &lt;= rows; i++) {
          $(&quot;#grid_gst&quot;).setValue($(&quot;#pan_no&quot;).getValue(), i, 5);
        }
}


$('#pan_no').setOnchange(panno);

//To uppercase Licence No.

$(&quot;#submit0000000001&quot;).find(&quot;button&quot;).on(&quot;click&quot; , function() {
 
  
  var rows = $(&quot;#grid_lictype&quot;).getNumberRows();
 
  
  for(var i=1;i&lt;=rows;i++)
    
   {
     //TO uppercase license No. for mobile
     var licno = $(&quot;#grid_lictype&quot;).getValue(i,2);
     $(&quot;#grid_lictype&quot;).setValue(licno.replace(/\w\S*/g, function(txt){return txt.toUpperCase();}),i,2);
     
     
   }
})
//Grid Hide and show based on GST is Register Check Box.
$(&quot;#is_gst_register&quot;).setOnchange(setGstGrid);
function setGstGrid() { 
  
if($(&quot;#is_gst_register&quot;).getValue() == 1)
{
  
  $(&quot;#grid_gst&quot;).show();
  $(&quot;#gstunregcust&quot;).hide();
    //$(&quot;#cst_no&quot;).hide();
  //$(&quot;#vat_tin_no&quot;).hide();
   //$(&quot;#cst_no&quot;).getControl().attr('Required', false); 
    //$(&quot;#vat_tin_no&quot;).getControl().attr('Required', false);  
  
}

  else{    
    $(&quot;#grid_gst&quot;).hide();
    $(&quot;#gstunregcust&quot;).show();
     var rows1 = $(&quot;#grid_gst&quot;).getNumberRows();
  
    for (var i= rows1; i >= 1; i--) {
   		$(&quot;#grid_gst&quot;).deleteRow(i);
   		//The first row can't be deleted, so clear the fields in the first row:
     	/*for (var j = 1; j &lt; 6; j++) {
     	  jQuery(&quot;#grid_lictype&quot;).setValue(&quot;&quot;,j,1);
          jQuery(&quot;#grid_lictype&quot;).setValue(&quot;&quot;,j,2);
          jQuery(&quot;#grid_lictype&quot;).setValue(&quot;&quot;,j,3);
          jQuery(&quot;#grid_lictype&quot;).setValue(&quot;&quot;,j,4);
          jQuery(&quot;#grid_lictype&quot;).setValue(&quot;&quot;,j,5);
     		//$(&quot;#gridTransact&quot;).setText(&quot;&quot;, 1, j);
     		//getFieldById(&quot;gridTransact&quot;).setValue(&quot;&quot;, 1, j);
   		}*/
   }
    //$(&quot;#cst_no&quot;).show();
    // $(&quot;#vat_tin_no&quot;).show();
    //$(&quot;#cst_no&quot;).getControl().attr('Required', true); 
    //$(&quot;#vat_tin_no&quot;).getControl().attr('Required', true); 
    //return false;
  }
}

//on save button--it will check GST Register is check or not if check then all grids of GSTIN is mendatory
$(&quot;#submit0000000001&quot;).find(&quot;button&quot;).on(&quot;click&quot; , function() {

  var rows = $(&quot;#grid_gst&quot;).getNumberRows();
   // this variable for comparing pan no to below gst grid
  //var panno=$(&quot;#pan_no&quot;).getValue();
 
  
  if($(&quot;#is_gst_register&quot;).getValue() == 1)
{
  for(var i=1;i&lt;=rows;i++)
    
   {
     
     //GST Related Validation
     var statecode =$(&quot;#grid_gst&quot;).getText(i,3);
     
     var gstinno =$(&quot;#grid_gst&quot;).getValue(i,4);
     
     //alert(gstinno);
       
     
     var gstno = $(&quot;#grid_gst&quot;).getValue(i,4);
    //Uppercase GST Unique ID
     $(&quot;#grid_gst&quot;).setValue(gstno.replace(/\w\S*/g, function(txt){return txt.toUpperCase();}),i,4);
     
     if($(&quot;#grid_gst&quot;).getValue(i,1 ) == &quot;&quot; || $(&quot;#grid_gst&quot;).getValue(i,2 ) == &quot;&quot; || $(&quot;#grid_gst&quot;).getValue(i,4 ) == &quot;&quot;){
       
       //alert(&quot;Please Enter Registerd State&quot;);
       //alert(&quot;OK&quot;);
       
        window.dynaform.flashMessage( {
       duration : 8000,
       emphasisMessage: &quot;Warning:&quot;,
       message:&quot;All fields are mendatory when GST Register is Checked,Please enter all GSTIN fields&quot;,
       type : 'danger',
       appendTo:$(&quot;#grid_gst&quot;),
       //absoluteTop : true   
        
    } );
      
       
       return false;
    
       }
     
     
     var n = gstinno.startsWith(statecode);
  
     //alert(n);
     
     if(n == false){
       
     alert(&quot;First 2 digit of GSTIN Unique Code should be same as State Code&quot;);
     return false;
     
     }
     
     //return false;
     var panno=$(&quot;#pan_no&quot;).getValue();
     
     //alert(panno);
    // var str = &quot;Hello world!&quot;;
     var gstinno_panno = gstinno.substring(2, 12);
     //alert(gstinno_panno);
     //return false;
     
     if(panno !== gstinno_panno){
       
       alert(&quot;PAN No. should match 10 digit of GSTIN Unique Code after the first 2 digit of State Code&quot;);
       
       return false;
       
       }
       
   }
}
  
})

//at the time of adding row  Header PAN NO  will set in GSTIN grid
$(&quot;#grid_gst&quot;).onAddRow(function(aNewRow, oGrid, rowIndex) {

    aNewRow[4].setValue($(&quot;#pan_no&quot;).getValue());


})



id(&quot;ext-gen60&quot;)</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ext-gen60&quot;)</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_(wasm.001 in devtest)/iframe_openCaseFrame</value>
   </webElementProperties>
</WebElementEntity>

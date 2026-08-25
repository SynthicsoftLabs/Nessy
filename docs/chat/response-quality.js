/* Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. */
(function(){'use strict';
var LOOP_PHRASES=[
  /what(?:'s| is) the task/i,
  /what(?:'s| is) the task you'd like to (?:perform|do)/i,
  /what (?:would|do) you like to (?:perform|do)/i,
  /please (?:provide|give) (?:the )?(?:details|requirements)/i,
  /do you have a task in mind/i,
  /what(?:'s| is) the task you'd like/i,
  /how can i assist you today/i,
  /what(?: would you like me to| should i) (?:do|perform) (?:first)?/i
];
var NORMALIZE_SPACE=/\s+/g;
function clean(text){
  var value=String(text==null?'':text).trim();
  if(value.length>=2 && ((value[0]==='"' && value[value.length-1]==='"') || (value[0]==='\'' && value[value.length-1]==='\''))){
    value=value.slice(1,-1).trim();
  }
  return value.replace(NORMALIZE_SPACE,' ');
}
function evaluate(text){
  var value=clean(text), reasons=[];
  if(!value) reasons.push('empty');
  for(var i=0;i<LOOP_PHRASES.length;i++) if(LOOP_PHRASES[i].test(value)){reasons.push('conversation-loop');break;}
  if(value.length<48 && /\?$/.test(value) && /assist|task|details|requirements/i.test(value)) reasons.push('low-information-question');
  return {lowQuality:reasons.length>0,reasons:reasons,text:value};
}
window.NessyResponseQuality={clean:clean,evaluate:evaluate};
})();

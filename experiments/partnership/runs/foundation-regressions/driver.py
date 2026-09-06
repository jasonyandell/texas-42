import os,pathlib,subprocess
root=pathlib.Path.cwd();deps=root/'walt/target/release/deps'
for name in ['kernel_sampler','solver_sigma1_repair','solver_policy','solver_viewer_fiber','solver_ordering']:
 cmd=['rustc','--edition=2021','--test',str(root/f'walt/walt/tests/{name}.rs'),'-L','dependency='+str(deps),'-C','opt-level=3','-C','overflow-checks=yes']
 for lib in ['walt','num_rational','num_traits','num_bigint']:
  libs=list(deps.glob('lib'+lib+'-*.rlib'));assert len(libs)==1
  cmd+=['--extern',lib+'='+str(libs[0])]
 target=root/'walt/target/release'/('foundation-'+name);cmd+=['-o',str(target)]
 subprocess.run(cmd,check=True,env={**os.environ,'CARGO_MANIFEST_DIR':str(root/'walt/walt')})
 subprocess.run([str(target),'--test-threads=1'],check=True,env={**os.environ,'RAYON_NUM_THREADS':'6'})

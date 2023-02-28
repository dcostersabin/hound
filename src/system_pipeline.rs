use crate::Apk;
use crate::Arch;
use crate::Composer;
use crate::Dpkg;
use crate::Gem;
use crate::Npm;
use crate::Package;
use crate::Pip;
use crate::Rpm;

pub struct SystemPipeline {
    pub packages: Vec<Package>,
}

impl SystemPipeline {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn start(&mut self) {
        self.run().await;
    }

    async fn run(&mut self) {
        self.arch().await;
        self.dpkg().await;
        self.pip().await;
        self.pip3().await;
        self.npm().await;
        self.composer().await;
        self.apk().await;
        self.gem().await;
        self.rpm().await;
    }

    async fn arch(&mut self) {
        let mut arch_obj = Arch::new();
        arch_obj.get_installed().await;
        self.packages.extend(arch_obj.packages);
    }

    async fn dpkg(&mut self) {
        let mut dpkg_obj = Dpkg::new();
        dpkg_obj.get_installed().await;
        self.packages.extend(dpkg_obj.packages);
    }

    async fn pip(&mut self) {
        let mut pip_obj = Pip::new();
        pip_obj.get_installed(false).await;
        self.packages.extend(pip_obj.packages);
    }

    async fn pip3(&mut self) {
        let mut pip_obj = Pip::new();
        pip_obj.get_installed(true).await;
        self.packages.extend(pip_obj.packages);
    }

    async fn npm(&mut self) {
        let mut npm_obj = Npm::new();
        npm_obj.get_installed().await;
        self.packages.extend(npm_obj.packages);
    }

    async fn composer(&mut self) {
        let mut composer_obj = Composer::new();
        composer_obj.get_installed().await;
        self.packages.extend(composer_obj.packages);
    }

    async fn apk(&mut self) {
        let mut apk_obj = Apk::new();
        apk_obj.get_installed().await;
        self.packages.extend(apk_obj.packages);
    }

    async fn gem(&mut self) {
        let mut gem_obj = Gem::new();
        gem_obj.get_installed().await;
        self.packages.extend(gem_obj.packages);
    }

    async fn rpm(&mut self) {
        let mut rpm_obj = Rpm::new();
        rpm_obj.get_installed().await;
        self.packages.extend(rpm_obj.packages);
    }
}
